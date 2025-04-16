use om_test_utils::config_generator::NodeConfigGenerator;
use std::{
    fs,
    path::PathBuf,
    process::{Command, Stdio},
    time::Duration,
};

// TODO: Migrate it to 1m-e2e CLI. Allow config of generator.
const WORK_DIR: &str = "/tmp/1m-network";
const NODE_NAMES: [&str; 4] = ["node0", "node1", "node2", "node3"];

const TEST_CHAIN_ID: u64 = 5321;
fn main() -> anyhow::Result<()> {
    let work_dir_root = PathBuf::from(WORK_DIR);
    eprintln!("WORK DIR ROOT = {}", work_dir_root.display());
    // Remove the root directory if it exists.
    let generator = NodeConfigGenerator::new(work_dir_root.clone()).add_nodes(NODE_NAMES);
    let genesis_certificate_file = format!("{WORK_DIR}/genesis_certificate.bcs");

    let operator_sk_file = generator.operator_privkey_file_path();
    let validator_set_file = generator.validator_set_file_path();
    generator.generate(TEST_CHAIN_ID);

    // Scan all the dirs in the parent directory.
    let entries = fs::read_dir(&work_dir_root).unwrap();
    eprintln!("Found entries in {}", work_dir_root.display());
    for (i, entry) in entries.enumerate() {
        let entry = entry.unwrap();
        eprintln!("Entry {i}: {}", entry.path().display());
    }

    // TODO: path as cli parameter instead of hardcoding as relative path.
    const NODE_BIN_PATH: &str = "./target/release/1m-node";
    let genesis_status = Command::new(NODE_BIN_PATH)
        .args([
            "genesis",
            "create",
            "--genesis.operator-privkey-path",
            &operator_sk_file.display().to_string(),
            "--genesis.validator-pubkeys",
            &validator_set_file.display().to_string(),
            "--genesis.output",
            &genesis_certificate_file,
        ])
        .status()
        .expect("Must create genesis certificate");

    if !genesis_status.success() {
        return Err(anyhow::anyhow!("Failed to create genesis certificate"));
    }

    // Start all nodes
    let mut node_processes = Vec::new();
    for node_dir in generator.node_dirs() {
        let node_config_path = NodeConfigGenerator::node_config_file_path(&node_dir);
        eprintln!(
            "Starting node: {} in background, node config path: {}",
            node_dir.display(),
            node_config_path.display()
        );

        let child = Command::new(NODE_BIN_PATH)
            .args([
                "--verbosity",
                "4",
                "validator",
                "--db.wipe",
                "--datadir",
                &node_dir.display().to_string(),
                "--config",
                // &node_dir.join(NODE_CONFIG_FILE).display().to_string(),
                &node_config_path.display().to_string(),
                "--genesis-certificate",
                &genesis_certificate_file,
            ])
            .stdout(Stdio::null())
            .spawn()
            .expect("Must start node");
        node_processes.push(child);
    }

    eprintln!("Check if all node processes are started");
    const MAX_STARTUP_ATTEMPTS: u32 = 30; // 30 seconds timeout
    const STARTUP_CHECK_INTERVAL: Duration = Duration::from_secs(1);

    let mut attempts = 0;
    loop {
        std::thread::sleep(STARTUP_CHECK_INTERVAL);
        match search_started_node_process() {
            Ok(node_count) => {
                if node_count == NODE_NAMES.len() {
                    break;
                }
            }
            Err(e) => {
                eprintln!("Node process status not ready: {}", e);
            }
        }
        attempts += 1;
        if attempts >= MAX_STARTUP_ATTEMPTS {
            return Err(anyhow::anyhow!("Timeout waiting for nodes to start"));
        }
    }
    // Although the processes are started, but it is not guaranteed that they are ready
    // for functional tests. So, we need to wait for some time.
    // TODO: find a way to do health check before running functional tests.
    eprintln!("Wait for 5 seconds before running functional tests");
    std::thread::sleep(Duration::from_secs(5));

    // Run functional tests
    let operator_secret =
        fs::read_to_string(operator_sk_file).expect("Must read operator secret key");

    const FUNCTIONAL_TEST_BIN_PATH: &str = "./target/release/1m-e2e";
    let test_status = Command::new(FUNCTIONAL_TEST_BIN_PATH)
        .args(["ft", "--operator-secret", &operator_secret])
        .status()
        .expect("Must run functional tests");

    if !test_status.success() {
        return Err(anyhow::anyhow!("Functional tests failed"));
    }

    // Gracefully terminate each node process
    for process in &mut node_processes {
        if let Err(e) = process.kill() {
            eprintln!("Failed to kill process: {}", e);
        }
    }

    eprintln!("Wait node processes exited");
    for mut p in node_processes {
        p.wait()?;
    }
    Ok(())
}

/// Return the number of started node processes. `1m-node` is the process name.
fn search_started_node_process() -> anyhow::Result<usize> {
    // Check all nodes processes started
    let pgrep_status = Command::new("pgrep").args(["1m-node"]).output()?;
    let pgrep_output = String::from_utf8(pgrep_status.stdout)?;
    eprintln!(
        "Found node processes: {}",
        pgrep_output.lines().collect::<Vec<&str>>().join(", ")
    );
    Ok(pgrep_output.lines().count())
}
