use alloy_sol_types::{sol, SolCall, SolError, SolEvent, SolStruct, SolType};

fn main() {
    println!("Hello, world!");
}

sol! {
    interface Token {
        #[derive(Debug, PartialEq)]
        function transfer(address recipient, uint amount) external;
    }

    #[derive(Debug, PartialEq)]
    struct MetaDataKeyValuePair {
        string key;
        string value;
    }

    #[derive(Debug, PartialEq)]
    enum AuthorityType {
        Master,
        Mint,
        Burn,
        Pause,
        Blacklist,
        UpdateMetadata,
    }

    interface TokenMint {
        #[derive(Debug, PartialEq)]
        function createNewToken(string symbol, uint8 decimals, address master_authority) external;

        #[derive(Debug, PartialEq)]
        function grantAuthority(AuthorityType authority_type, address new_authority, uint amount) external;

        #[derive(Debug, PartialEq)]
        function revokeAuthority(AuthorityType authority_type, address new_authority) external;

        #[derive(Debug, PartialEq)]
        function blacklistAccount(address account) external;

        #[derive(Debug, PartialEq)]
        function whitelistAccount(address account) external;

        #[derive(Debug, PartialEq)]
        function mintTo(uint amount, address account) external;

        #[derive(Debug, PartialEq)]
        function burnFromAccount(uint amount, address account) external;

        #[derive(Debug, PartialEq)]
        function closeAccount() external;

        #[derive(Debug, PartialEq)]
        function pause() external;

        #[derive(Debug, PartialEq)]
        function unpause() external;

        #[derive(Debug, PartialEq)]
        function updateMetadata(string name, string uri, MetaDataKeyValuePair[] additional_metadata) external;

        #[derive(Debug, PartialEq)]
        function transfer(address recipient, uint amount) external;

        // ERC-20 eth_call methods
        #[derive(Debug, PartialEq)]
        function name() external returns (string);

        #[derive(Debug, PartialEq)]
        function symbol() external returns (string);

        #[derive(Debug, PartialEq)]
        function decimals() external returns (uint8);

        #[derive(Debug, PartialEq)]
        function totalSupply() external returns (uint256);

        #[derive(Debug, PartialEq)]
        function balanceOf(address owner) external returns (uint256 balance);

        // ERC-20 eth_call events
        #[derive(Debug, PartialEq)]
        event Transfer(address _from, address _to, uint256 _value);

        #[derive(Debug, PartialEq)]
        event Failure(address _from, address _to, string message);

        #[derive(Debug, PartialEq)]
        event Success(address _from, address _to, string message);
    }
}
