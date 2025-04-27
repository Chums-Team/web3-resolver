pub const DOMAIN_ABI_JSON: &'static str = r#"
{
  "ABI version": 2,
  "version": "2.2",
  "header": ["pubkey", "time", "expire"],
  "functions": [
    {
      "name": "onDeployRetry",
      "id": "0x4A2E4FD6",
      "inputs": [
        {"name":"value0","type":"cell"},
        {"name":"params","type":"cell"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "getDurationConfig",
      "inputs": [
        {"name":"answerId","type":"uint32"}
      ],
      "outputs": [
        {"components":[{"name":"startZeroAuction","type":"uint32"},{"name":"expiring","type":"uint32"},{"name":"grace","type":"uint32"}],"name":"durationConfig","type":"tuple"}
      ]
    },
    {
      "name": "getConfig",
      "inputs": [
        {"name":"answerId","type":"uint32"}
      ],
      "outputs": [
        {"components":[{"name":"maxDuration","type":"uint32"},{"name":"graceFinePercent","type":"uint128"}],"name":"config","type":"tuple"}
      ]
    },
    {
      "name": "getPrice",
      "inputs": [
        {"name":"answerId","type":"uint32"}
      ],
      "outputs": [
        {"name":"price","type":"uint128"}
      ]
    },
    {
      "name": "getFlags",
      "inputs": [
        {"name":"answerId","type":"uint32"}
      ],
      "outputs": [
        {"name":"reserved","type":"bool"},
        {"name":"inZeroAuction","type":"bool"},
        {"name":"needZeroAuction","type":"bool"}
      ]
    },
    {
      "name": "getZeroAuction",
      "inputs": [
        {"name":"answerId","type":"uint32"}
      ],
      "outputs": [
        {"name":"zeroAuction","type":"optional(address)"}
      ]
    },
    {
      "name": "startZeroAuction",
      "inputs": [
        {"components":[{"name":"auctionRoot","type":"address"},{"name":"tokenRoot","type":"address"},{"name":"duration","type":"uint32"}],"name":"config","type":"tuple"},
        {"name":"amount","type":"uint128"},
        {"name":"sender","type":"address"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "changeOwner",
      "inputs": [
        {"name":"newOwner","type":"address"},
        {"name":"sendGasTo","type":"address"},
        {"components":[{"name":"value","type":"uint128"},{"name":"payload","type":"cell"}],"name":"callbacks","type":"map(address,tuple)"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "changeManager",
      "inputs": [
        {"name":"newManager","type":"address"},
        {"name":"sendGasTo","type":"address"},
        {"components":[{"name":"value","type":"uint128"},{"name":"payload","type":"cell"}],"name":"callbacks","type":"map(address,tuple)"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "transfer",
      "inputs": [
        {"name":"to","type":"address"},
        {"name":"sendGasTo","type":"address"},
        {"components":[{"name":"value","type":"uint128"},{"name":"payload","type":"cell"}],"name":"callbacks","type":"map(address,tuple)"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "expectedRenewAmount",
      "inputs": [
        {"name":"answerId","type":"uint32"},
        {"name":"newExpireTime","type":"uint32"}
      ],
      "outputs": [
        {"name":"amount","type":"uint128"}
      ]
    },
    {
      "name": "renew",
      "inputs": [
        {"name":"amount","type":"uint128"},
        {"name":"sender","type":"address"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "unreserve",
      "inputs": [
        {"name":"owner","type":"address"},
        {"name":"price","type":"uint128"},
        {"name":"expireTime","type":"uint32"},
        {"name":"needZeroAuction","type":"bool"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "getJson",
      "inputs": [
        {"name":"answerId","type":"uint32"}
      ],
      "outputs": [
        {"name":"json","type":"string"}
      ]
    },
    {
      "name": "supportsInterface",
      "inputs": [
        {"name":"answerId","type":"uint32"},
        {"name":"interfaceID","type":"uint32"}
      ],
      "outputs": [
        {"name":"support","type":"bool"}
      ]
    },
    {
      "name": "confiscate",
      "inputs": [
        {"name":"newOwner","type":"address"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "expire",
      "inputs": [
      ],
      "outputs": [
      ]
    },
    {
      "name": "afterCodeUpgrade",
      "inputs": [
      ],
      "outputs": [
      ]
    },
    {
      "name": "getPath",
      "inputs": [
        {"name":"answerId","type":"uint32"}
      ],
      "outputs": [
        {"name":"path","type":"string"}
      ]
    },
    {
      "name": "getDetails",
      "inputs": [
        {"name":"answerId","type":"uint32"}
      ],
      "outputs": [
        {"name":"owner","type":"address"},
        {"name":"initTime","type":"uint32"},
        {"name":"expireTime","type":"uint32"}
      ]
    },
    {
      "name": "getStatus",
      "inputs": [
        {"name":"answerId","type":"uint32"}
      ],
      "outputs": [
        {"name":"status","type":"uint8"}
      ]
    },
    {
      "name": "resolve",
      "inputs": [
        {"name":"answerId","type":"uint32"}
      ],
      "outputs": [
        {"name":"target","type":"address"}
      ]
    },
    {
      "name": "query",
      "inputs": [
        {"name":"answerId","type":"uint32"},
        {"name":"key","type":"uint32"}
      ],
      "outputs": [
        {"name":"value","type":"optional(cell)"}
      ]
    },
    {
      "name": "getRecords",
      "inputs": [
        {"name":"answerId","type":"uint32"}
      ],
      "outputs": [
        {"name":"records","type":"map(uint32,cell)"}
      ]
    },
    {
      "name": "setTarget",
      "inputs": [
        {"name":"target","type":"address"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "setRecords",
      "inputs": [
        {"name":"records","type":"map(uint32,cell)"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "setRecord",
      "inputs": [
        {"name":"key","type":"uint32"},
        {"name":"value","type":"cell"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "deleteRecords",
      "inputs": [
        {"name":"keys","type":"uint32[]"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "deleteRecord",
      "inputs": [
        {"name":"key","type":"uint32"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "createSubdomain",
      "inputs": [
        {"name":"name","type":"string"},
        {"name":"owner","type":"address"},
        {"name":"renewable","type":"bool"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "renewSubdomain",
      "inputs": [
        {"name":"subdomain","type":"address"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "requestUpgrade",
      "inputs": [
      ],
      "outputs": [
      ]
    },
    {
      "name": "acceptUpgrade",
      "inputs": [
        {"name":"sid","type":"uint16"},
        {"components":[{"name":"major","type":"uint32"},{"name":"minor","type":"uint32"}],"name":"version","type":"tuple"},
        {"name":"code","type":"cell"},
        {"name":"params","type":"cell"},
        {"name":"remainingGasTo","type":"address"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "getSID",
      "inputs": [
        {"name":"answerId","type":"uint32"}
      ],
      "outputs": [
        {"name":"sid","type":"uint16"}
      ]
    },
    {
      "name": "getVersion",
      "inputs": [
        {"name":"answerId","type":"uint32"}
      ],
      "outputs": [
        {"components":[{"name":"major","type":"uint32"},{"name":"minor","type":"uint32"}],"name":"version","type":"tuple"}
      ]
    },
    {
      "name": "indexCode",
      "inputs": [
        {"name":"answerId","type":"uint32"}
      ],
      "outputs": [
        {"name":"code","type":"cell"}
      ]
    },
    {
      "name": "indexCodeHash",
      "inputs": [
        {"name":"answerId","type":"uint32"}
      ],
      "outputs": [
        {"name":"hash","type":"uint256"}
      ]
    },
    {
      "name": "resolveIndex",
      "inputs": [
        {"name":"answerId","type":"uint32"},
        {"name":"collection","type":"address"},
        {"name":"owner","type":"address"}
      ],
      "outputs": [
        {"name":"index","type":"address"}
      ]
    },
    {
      "name": "getInfo",
      "inputs": [
        {"name":"answerId","type":"uint32"}
      ],
      "outputs": [
        {"name":"id","type":"uint256"},
        {"name":"owner","type":"address"},
        {"name":"manager","type":"address"},
        {"name":"collection","type":"address"}
      ]
    },
    {
      "name": "constructor",
      "inputs": [
      ],
      "outputs": [
      ]
    },
    {
      "name": "_owner",
      "inputs": [
      ],
      "outputs": [
        {"name":"_owner","type":"address"}
      ]
    },
    {
      "name": "_manager",
      "inputs": [
      ],
      "outputs": [
        {"name":"_manager","type":"address"}
      ]
    },
    {
      "name": "_indexCode",
      "inputs": [
      ],
      "outputs": [
        {"name":"_indexCode","type":"cell"}
      ]
    },
    {
      "name": "_sid",
      "inputs": [
      ],
      "outputs": [
        {"name":"_sid","type":"uint16"}
      ]
    },
    {
      "name": "_version",
      "inputs": [
      ],
      "outputs": [
        {"components":[{"name":"major","type":"uint32"},{"name":"minor","type":"uint32"}],"name":"_version","type":"tuple"}
      ]
    },
    {
      "name": "_id",
      "inputs": [
      ],
      "outputs": [
        {"name":"_id","type":"uint256"}
      ]
    },
    {
      "name": "_root",
      "inputs": [
      ],
      "outputs": [
        {"name":"_root","type":"address"}
      ]
    },
    {
      "name": "_path",
      "inputs": [
      ],
      "outputs": [
        {"name":"_path","type":"string"}
      ]
    },
    {
      "name": "_initTime",
      "inputs": [
      ],
      "outputs": [
        {"name":"_initTime","type":"uint32"}
      ]
    },
    {
      "name": "_expireTime",
      "inputs": [
      ],
      "outputs": [
        {"name":"_expireTime","type":"uint32"}
      ]
    },
    {
      "name": "_target",
      "inputs": [
      ],
      "outputs": [
        {"name":"_target","type":"address"}
      ]
    },
    {
      "name": "_records",
      "inputs": [
      ],
      "outputs": [
        {"name":"_records","type":"map(uint32,cell)"}
      ]
    },
    {
      "name": "_durationConfig",
      "inputs": [
      ],
      "outputs": [
        {"components":[{"name":"startZeroAuction","type":"uint32"},{"name":"expiring","type":"uint32"},{"name":"grace","type":"uint32"}],"name":"_durationConfig","type":"tuple"}
      ]
    },
    {
      "name": "_config",
      "inputs": [
      ],
      "outputs": [
        {"components":[{"name":"maxDuration","type":"uint32"},{"name":"graceFinePercent","type":"uint128"}],"name":"_config","type":"tuple"}
      ]
    },
    {
      "name": "_price",
      "inputs": [
      ],
      "outputs": [
        {"name":"_price","type":"uint128"}
      ]
    },
    {
      "name": "_reserved",
      "inputs": [
      ],
      "outputs": [
        {"name":"_reserved","type":"bool"}
      ]
    },
    {
      "name": "_inZeroAuction",
      "inputs": [
      ],
      "outputs": [
        {"name":"_inZeroAuction","type":"bool"}
      ]
    },
    {
      "name": "_needZeroAuction",
      "inputs": [
      ],
      "outputs": [
        {"name":"_needZeroAuction","type":"bool"}
      ]
    },
    {
      "name": "_auctionRoot",
      "inputs": [
      ],
      "outputs": [
        {"name":"_auctionRoot","type":"address"}
      ]
    },
    {
      "name": "_zeroAuction",
      "inputs": [
      ],
      "outputs": [
        {"name":"_zeroAuction","type":"address"}
      ]
    },
    {
      "name": "_initialAmount",
      "inputs": [
      ],
      "outputs": [
        {"name":"_initialAmount","type":"uint128"}
      ]
    },
    {
      "name": "_initialOwner",
      "inputs": [
      ],
      "outputs": [
        {"name":"_initialOwner","type":"address"}
      ]
    }
  ],
  "data": [
  ],
  "events": [
    {
      "name": "ZeroAuctionStarted",
      "inputs": [
        {"name":"zeroAuction","type":"address"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "ZeroAuctionFinished",
      "inputs": [
        {"name":"newOwner","type":"address"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "Renewed",
      "inputs": [
        {"name":"time","type":"uint32"},
        {"name":"duration","type":"uint32"},
        {"name":"newExpireTime","type":"uint32"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "ChangedTarget",
      "inputs": [
        {"name":"oldTarget","type":"address"},
        {"name":"newTarget","type":"address"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "ChangedOwner",
      "inputs": [
        {"name":"oldOwner","type":"address"},
        {"name":"newOwner","type":"address"},
        {"name":"confiscate","type":"bool"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "CodeUpgraded",
      "inputs": [
        {"components":[{"name":"major","type":"uint32"},{"name":"minor","type":"uint32"}],"name":"oldVersion","type":"tuple"},
        {"components":[{"name":"major","type":"uint32"},{"name":"minor","type":"uint32"}],"name":"newVersion","type":"tuple"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "NftCreated",
      "inputs": [
        {"name":"id","type":"uint256"},
        {"name":"owner","type":"address"},
        {"name":"manager","type":"address"},
        {"name":"collection","type":"address"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "OwnerChanged",
      "inputs": [
        {"name":"oldOwner","type":"address"},
        {"name":"newOwner","type":"address"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "ManagerChanged",
      "inputs": [
        {"name":"oldManager","type":"address"},
        {"name":"newManager","type":"address"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "NftBurned",
      "inputs": [
        {"name":"id","type":"uint256"},
        {"name":"owner","type":"address"},
        {"name":"manager","type":"address"},
        {"name":"collection","type":"address"}
      ],
      "outputs": [
      ]
    }
  ],
  "fields": [
    {"name":"_pubkey","type":"uint256"},
    {"name":"_timestamp","type":"uint64"},
    {"name":"_constructorFlag","type":"bool"},
    {"name":"_owner","type":"address"},
    {"name":"_manager","type":"address"},
    {"name":"_indexCode","type":"cell"},
    {"name":"_sid","type":"uint16"},
    {"components":[{"name":"major","type":"uint32"},{"name":"minor","type":"uint32"}],"name":"_version","type":"tuple"},
    {"name":"_id","type":"uint256"},
    {"name":"_root","type":"address"},
    {"name":"_path","type":"string"},
    {"name":"_initTime","type":"uint32"},
    {"name":"_expireTime","type":"uint32"},
    {"name":"_target","type":"address"},
    {"name":"_records","type":"map(uint32,cell)"},
    {"components":[{"name":"startZeroAuction","type":"uint32"},{"name":"expiring","type":"uint32"},{"name":"grace","type":"uint32"}],"name":"_durationConfig","type":"tuple"},
    {"components":[{"name":"maxDuration","type":"uint32"},{"name":"graceFinePercent","type":"uint128"}],"name":"_config","type":"tuple"},
    {"name":"_price","type":"uint128"},
    {"name":"_reserved","type":"bool"},
    {"name":"_inZeroAuction","type":"bool"},
    {"name":"_needZeroAuction","type":"bool"},
    {"name":"_auctionRoot","type":"address"},
    {"name":"_zeroAuction","type":"address"},
    {"name":"_initialAmount","type":"uint128"},
    {"name":"_initialOwner","type":"address"}
  ]
}
"#;

pub const ROOT_ABI_JSON: &'static str = r#"
{
  "ABI version": 2,
  "version": "2.2",
  "header": ["pubkey", "time", "expire"],
  "functions": [
    {
      "name": "constructor",
      "inputs": [
        {"name":"domainCode","type":"cell"},
        {"name":"subdomainCode","type":"cell"},
        {"name":"indexBasisCode","type":"cell"},
        {"name":"indexCode","type":"cell"},
        {"name":"json","type":"string"},
        {"name":"platformCode","type":"cell"},
        {"name":"dao","type":"address"},
        {"name":"admin","type":"address"},
        {"components":[{"name":"maxNameLength","type":"uint32"},{"name":"maxPathLength","type":"uint32"},{"name":"minDuration","type":"uint32"},{"name":"maxDuration","type":"uint32"},{"name":"graceFinePercent","type":"uint128"},{"name":"startZeroAuctionFee","type":"uint128"}],"name":"config","type":"tuple"},
        {"components":[{"name":"longPrice","type":"uint128"},{"name":"shortPrices","type":"uint128[]"},{"name":"onlyLettersFeePercent","type":"uint128"},{"name":"needZeroAuctionLength","type":"uint32"}],"name":"priceConfig","type":"tuple"},
        {"components":[{"name":"auctionRoot","type":"address"},{"name":"tokenRoot","type":"address"},{"name":"duration","type":"uint32"}],"name":"auctionConfig","type":"tuple"},
        {"components":[{"name":"startZeroAuction","type":"uint32"},{"name":"expiring","type":"uint32"},{"name":"grace","type":"uint32"}],"name":"durationConfig","type":"tuple"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "getPath",
      "inputs": [
        {"name":"answerId","type":"uint32"}
      ],
      "outputs": [
        {"name":"path","type":"string"}
      ]
    },
    {
      "name": "getDetails",
      "inputs": [
        {"name":"answerId","type":"uint32"}
      ],
      "outputs": [
        {"name":"tld","type":"string"},
        {"name":"dao","type":"address"},
        {"name":"active","type":"bool"}
      ]
    },
    {
      "name": "getConfigs",
      "inputs": [
        {"name":"answerId","type":"uint32"}
      ],
      "outputs": [
        {"components":[{"name":"maxNameLength","type":"uint32"},{"name":"maxPathLength","type":"uint32"},{"name":"minDuration","type":"uint32"},{"name":"maxDuration","type":"uint32"},{"name":"graceFinePercent","type":"uint128"},{"name":"startZeroAuctionFee","type":"uint128"}],"name":"config","type":"tuple"},
        {"components":[{"name":"longPrice","type":"uint128"},{"name":"shortPrices","type":"uint128[]"},{"name":"onlyLettersFeePercent","type":"uint128"},{"name":"needZeroAuctionLength","type":"uint32"}],"name":"priceConfig","type":"tuple"},
        {"components":[{"name":"auctionRoot","type":"address"},{"name":"tokenRoot","type":"address"},{"name":"duration","type":"uint32"}],"name":"auctionConfig","type":"tuple"},
        {"components":[{"name":"startZeroAuction","type":"uint32"},{"name":"expiring","type":"uint32"},{"name":"grace","type":"uint32"}],"name":"durationConfig","type":"tuple"}
      ]
    },
    {
      "name": "checkName",
      "inputs": [
        {"name":"answerId","type":"uint32"},
        {"name":"name","type":"string"}
      ],
      "outputs": [
        {"name":"correct","type":"bool"}
      ]
    },
    {
      "name": "expectedPrice",
      "inputs": [
        {"name":"answerId","type":"uint32"},
        {"name":"name","type":"string"}
      ],
      "outputs": [
        {"name":"price","type":"uint128"},
        {"name":"needZeroAuction","type":"bool"}
      ]
    },
    {
      "name": "expectedRegisterAmount",
      "inputs": [
        {"name":"answerId","type":"uint32"},
        {"name":"name","type":"string"},
        {"name":"duration","type":"uint32"}
      ],
      "outputs": [
        {"name":"amount","type":"uint128"}
      ]
    },
    {
      "name": "resolve",
      "inputs": [
        {"name":"answerId","type":"uint32"},
        {"name":"path","type":"string"}
      ],
      "outputs": [
        {"name":"certificate","type":"address"}
      ]
    },
    {
      "name": "expectedCertificateCodeHash",
      "inputs": [
        {"name":"answerId","type":"uint32"},
        {"name":"target","type":"address"},
        {"name":"sid","type":"uint16"}
      ],
      "outputs": [
        {"name":"codeHash","type":"uint256"}
      ]
    },
    {
      "name": "buildRegisterPayload",
      "inputs": [
        {"name":"answerId","type":"uint32"},
        {"name":"name","type":"string"}
      ],
      "outputs": [
        {"name":"payload","type":"cell"}
      ]
    },
    {
      "name": "buildRenewPayload",
      "inputs": [
        {"name":"answerId","type":"uint32"},
        {"name":"name","type":"string"}
      ],
      "outputs": [
        {"name":"payload","type":"cell"}
      ]
    },
    {
      "name": "buildStartZeroAuctionPayload",
      "inputs": [
        {"name":"answerId","type":"uint32"},
        {"name":"name","type":"string"}
      ],
      "outputs": [
        {"name":"payload","type":"cell"}
      ]
    },
    {
      "name": "onAcceptTokensTransfer",
      "inputs": [
        {"name":"value0","type":"address"},
        {"name":"amount","type":"uint128"},
        {"name":"sender","type":"address"},
        {"name":"value3","type":"address"},
        {"name":"value4","type":"address"},
        {"name":"payload","type":"cell"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "zeroAuctionInitialBid",
      "inputs": [
        {"name":"path","type":"string"},
        {"name":"auction","type":"address"},
        {"name":"initialAmount","type":"uint128"},
        {"name":"initialOwner","type":"address"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "returnTokensFromDomain",
      "inputs": [
        {"name":"path","type":"string"},
        {"name":"amount","type":"uint128"},
        {"name":"recipient","type":"address"},
        {"name":"reason","type":"uint8"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "deploySubdomain",
      "inputs": [
        {"name":"path","type":"string"},
        {"name":"name","type":"string"},
        {"components":[{"name":"owner","type":"address"},{"name":"creator","type":"address"},{"name":"expireTime","type":"uint32"},{"name":"parent","type":"address"},{"name":"renewable","type":"bool"}],"name":"setup","type":"tuple"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "confiscate",
      "inputs": [
        {"name":"path","type":"string"},
        {"name":"reason","type":"string"},
        {"name":"owner","type":"address"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "reserve",
      "inputs": [
        {"name":"paths","type":"string[]"},
        {"name":"reason","type":"string"},
        {"name":"owner","type":"optional(address)"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "unreserve",
      "inputs": [
        {"name":"path","type":"string"},
        {"name":"reason","type":"string"},
        {"name":"owner","type":"address"},
        {"name":"price","type":"uint128"},
        {"name":"expireTime","type":"uint32"},
        {"name":"needZeroAuction","type":"bool"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "activate",
      "inputs": [
      ],
      "outputs": [
      ]
    },
    {
      "name": "deactivate",
      "inputs": [
      ],
      "outputs": [
      ]
    },
    {
      "name": "burnBalance",
      "inputs": [
        {"name":"amount","type":"uint128"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "syncBalance",
      "inputs": [
      ],
      "outputs": [
      ]
    },
    {
      "name": "changePriceConfig",
      "inputs": [
        {"components":[{"name":"longPrice","type":"uint128"},{"name":"shortPrices","type":"uint128[]"},{"name":"onlyLettersFeePercent","type":"uint128"},{"name":"needZeroAuctionLength","type":"uint32"}],"name":"priceConfig","type":"tuple"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "changeConfigs",
      "inputs": [
        {"components":[{"name":"maxNameLength","type":"uint32"},{"name":"maxPathLength","type":"uint32"},{"name":"minDuration","type":"uint32"},{"name":"maxDuration","type":"uint32"},{"name":"graceFinePercent","type":"uint128"},{"name":"startZeroAuctionFee","type":"uint128"}],"name":"config","type":"optional(tuple)"},
        {"components":[{"name":"auctionRoot","type":"address"},{"name":"tokenRoot","type":"address"},{"name":"duration","type":"uint32"}],"name":"auctionConfig","type":"optional(tuple)"},
        {"components":[{"name":"startZeroAuction","type":"uint32"},{"name":"expiring","type":"uint32"},{"name":"grace","type":"uint32"}],"name":"durationConfig","type":"optional(tuple)"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "changeAdmin",
      "inputs": [
        {"name":"admin","type":"address"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "changeDao",
      "inputs": [
        {"name":"dao","type":"address"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "upgradeToLatest",
      "inputs": [
        {"name":"sid","type":"uint16"},
        {"name":"destination","type":"address"},
        {"name":"remainingGasTo","type":"address"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "upgradeToSpecific",
      "inputs": [
        {"name":"sid","type":"uint16"},
        {"name":"destination","type":"address"},
        {"components":[{"name":"major","type":"uint32"},{"name":"minor","type":"uint32"}],"name":"version","type":"tuple"},
        {"name":"code","type":"cell"},
        {"name":"params","type":"cell"},
        {"name":"remainingGasTo","type":"address"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "setVersionActivation",
      "inputs": [
        {"name":"sid","type":"uint16"},
        {"components":[{"name":"major","type":"uint32"},{"name":"minor","type":"uint32"}],"name":"version","type":"tuple"},
        {"name":"active","type":"bool"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "createNewDomainVersion",
      "inputs": [
        {"name":"minor","type":"bool"},
        {"name":"code","type":"cell"},
        {"name":"params","type":"cell"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "createNewSubdomainVersion",
      "inputs": [
        {"name":"minor","type":"bool"},
        {"name":"code","type":"cell"},
        {"name":"params","type":"cell"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "upgrade",
      "inputs": [
        {"name":"code","type":"cell"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "getSIDs",
      "inputs": [
        {"name":"answerId","type":"uint32"}
      ],
      "outputs": [
        {"name":"sids","type":"uint16[]"}
      ]
    },
    {
      "name": "getSlaveData",
      "inputs": [
        {"name":"answerId","type":"uint32"},
        {"name":"sid","type":"uint16"}
      ],
      "outputs": [
        {"name":"code","type":"cell"},
        {"name":"params","type":"cell"},
        {"components":[{"name":"major","type":"uint32"},{"name":"minor","type":"uint32"}],"name":"latest","type":"tuple"},
        {"name":"versionsCount","type":"uint32"}
      ]
    },
    {
      "name": "getSlaveVersions",
      "inputs": [
        {"name":"answerId","type":"uint32"},
        {"name":"sid","type":"uint16"}
      ],
      "outputs": [
        {"components":[{"name":"major","type":"uint32"},{"name":"minor","type":"uint32"}],"name":"versions","type":"tuple[]"}
      ]
    },
    {
      "name": "getSlaveVersion",
      "inputs": [
        {"name":"answerId","type":"uint32"},
        {"name":"sid","type":"uint16"},
        {"components":[{"name":"major","type":"uint32"},{"name":"minor","type":"uint32"}],"name":"version","type":"tuple"}
      ],
      "outputs": [
        {"components":[{"name":"hash","type":"uint256"},{"name":"active","type":"bool"}],"name":"data","type":"tuple"}
      ]
    },
    {
      "name": "onWalletDeployed",
      "inputs": [
        {"name":"wallet","type":"address"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "getToken",
      "inputs": [
        {"name":"answerId","type":"uint32"}
      ],
      "outputs": [
        {"name":"token","type":"address"}
      ]
    },
    {
      "name": "getWallet",
      "inputs": [
        {"name":"answerId","type":"uint32"}
      ],
      "outputs": [
        {"name":"wallet","type":"address"}
      ]
    },
    {
      "name": "getBalance",
      "inputs": [
        {"name":"answerId","type":"uint32"}
      ],
      "outputs": [
        {"name":"balance","type":"uint128"}
      ]
    },
    {
      "name": "onReceiveBalance",
      "inputs": [
        {"name":"balance","type":"uint128"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "nftAddress",
      "inputs": [
        {"name":"answerId","type":"uint32"},
        {"name":"id","type":"uint256"}
      ],
      "outputs": [
        {"name":"nft","type":"address"}
      ]
    },
    {
      "name": "supportsInterface",
      "inputs": [
        {"name":"answerId","type":"uint32"},
        {"name":"interfaceID","type":"uint32"}
      ],
      "outputs": [
        {"name":"support","type":"bool"}
      ]
    },
    {
      "name": "onMint",
      "inputs": [
        {"name":"id","type":"uint256"},
        {"name":"owner","type":"address"},
        {"name":"manager","type":"address"},
        {"name":"creator","type":"address"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "onBurn",
      "inputs": [
        {"name":"id","type":"uint256"},
        {"name":"owner","type":"address"},
        {"name":"manager","type":"address"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "calcAddress",
      "inputs": [
        {"name":"stateInit","type":"cell"}
      ],
      "outputs": [
        {"name":"value0","type":"address"}
      ]
    },
    {
      "name": "getJson",
      "inputs": [
        {"name":"answerId","type":"uint32"}
      ],
      "outputs": [
        {"name":"json","type":"string"}
      ]
    },
    {
      "name": "indexBasisCode",
      "inputs": [
        {"name":"answerId","type":"uint32"}
      ],
      "outputs": [
        {"name":"code","type":"cell"}
      ]
    },
    {
      "name": "indexBasisCodeHash",
      "inputs": [
        {"name":"answerId","type":"uint32"}
      ],
      "outputs": [
        {"name":"hash","type":"uint256"}
      ]
    },
    {
      "name": "indexCode",
      "inputs": [
        {"name":"answerId","type":"uint32"}
      ],
      "outputs": [
        {"name":"code","type":"cell"}
      ]
    },
    {
      "name": "indexCodeHash",
      "inputs": [
        {"name":"answerId","type":"uint32"}
      ],
      "outputs": [
        {"name":"hash","type":"uint256"}
      ]
    },
    {
      "name": "resolveIndexBasis",
      "inputs": [
        {"name":"answerId","type":"uint32"}
      ],
      "outputs": [
        {"name":"indexBasis","type":"address"}
      ]
    },
    {
      "name": "totalSupply",
      "inputs": [
        {"name":"answerId","type":"uint32"}
      ],
      "outputs": [
        {"name":"count","type":"uint128"}
      ]
    },
    {
      "name": "nftCode",
      "inputs": [
        {"name":"answerId","type":"uint32"}
      ],
      "outputs": [
        {"name":"code","type":"cell"}
      ]
    },
    {
      "name": "nftCodeHash",
      "inputs": [
        {"name":"answerId","type":"uint32"}
      ],
      "outputs": [
        {"name":"codeHash","type":"uint256"}
      ]
    },
    {
      "name": "_totalSupply",
      "inputs": [
      ],
      "outputs": [
        {"name":"_totalSupply","type":"uint128"}
      ]
    },
    {
      "name": "_nftCode",
      "inputs": [
      ],
      "outputs": [
        {"name":"_nftCode","type":"cell"}
      ]
    },
    {
      "name": "_indexBasisCode",
      "inputs": [
      ],
      "outputs": [
        {"name":"_indexBasisCode","type":"cell"}
      ]
    },
    {
      "name": "_indexCode",
      "inputs": [
      ],
      "outputs": [
        {"name":"_indexCode","type":"cell"}
      ]
    },
    {
      "name": "_json",
      "inputs": [
      ],
      "outputs": [
        {"name":"_json","type":"string"}
      ]
    },
    {
      "name": "_token",
      "inputs": [
      ],
      "outputs": [
        {"name":"_token","type":"address"}
      ]
    },
    {
      "name": "_wallet",
      "inputs": [
      ],
      "outputs": [
        {"name":"_wallet","type":"address"}
      ]
    },
    {
      "name": "_balance",
      "inputs": [
      ],
      "outputs": [
        {"name":"_balance","type":"uint128"}
      ]
    },
    {
      "name": "_randomNonce",
      "inputs": [
      ],
      "outputs": [
        {"name":"_randomNonce","type":"uint256"}
      ]
    },
    {
      "name": "_tld",
      "inputs": [
      ],
      "outputs": [
        {"name":"_tld","type":"string"}
      ]
    },
    {
      "name": "_dao",
      "inputs": [
      ],
      "outputs": [
        {"name":"_dao","type":"address"}
      ]
    },
    {
      "name": "_admin",
      "inputs": [
      ],
      "outputs": [
        {"name":"_admin","type":"address"}
      ]
    },
    {
      "name": "_active",
      "inputs": [
      ],
      "outputs": [
        {"name":"_active","type":"bool"}
      ]
    },
    {
      "name": "_config",
      "inputs": [
      ],
      "outputs": [
        {"components":[{"name":"maxNameLength","type":"uint32"},{"name":"maxPathLength","type":"uint32"},{"name":"minDuration","type":"uint32"},{"name":"maxDuration","type":"uint32"},{"name":"graceFinePercent","type":"uint128"},{"name":"startZeroAuctionFee","type":"uint128"}],"name":"_config","type":"tuple"}
      ]
    },
    {
      "name": "_priceConfig",
      "inputs": [
      ],
      "outputs": [
        {"components":[{"name":"longPrice","type":"uint128"},{"name":"shortPrices","type":"uint128[]"},{"name":"onlyLettersFeePercent","type":"uint128"},{"name":"needZeroAuctionLength","type":"uint32"}],"name":"_priceConfig","type":"tuple"}
      ]
    },
    {
      "name": "_auctionConfig",
      "inputs": [
      ],
      "outputs": [
        {"components":[{"name":"auctionRoot","type":"address"},{"name":"tokenRoot","type":"address"},{"name":"duration","type":"uint32"}],"name":"_auctionConfig","type":"tuple"}
      ]
    },
    {
      "name": "_durationConfig",
      "inputs": [
      ],
      "outputs": [
        {"components":[{"name":"startZeroAuction","type":"uint32"},{"name":"expiring","type":"uint32"},{"name":"grace","type":"uint32"}],"name":"_durationConfig","type":"tuple"}
      ]
    }
  ],
  "data": [
    {"key":1,"name":"_randomNonce","type":"uint256"},
    {"key":2,"name":"_tld","type":"string"}
  ],
  "events": [
    {
      "name": "Renewed",
      "inputs": [
        {"name":"path","type":"string"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "ZeroAuctionStarted",
      "inputs": [
        {"name":"path","type":"string"},
        {"name":"auction","type":"address"},
        {"name":"initialAmount","type":"uint128"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "Confiscated",
      "inputs": [
        {"name":"path","type":"string"},
        {"name":"reason","type":"string"},
        {"name":"owner","type":"address"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "Reserved",
      "inputs": [
        {"name":"path","type":"string"},
        {"name":"reason","type":"string"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "Unreserved",
      "inputs": [
        {"name":"path","type":"string"},
        {"name":"reason","type":"string"},
        {"name":"owner","type":"address"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "DomainCodeUpgraded",
      "inputs": [
        {"name":"newVersion","type":"uint16"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "CodeUpgraded",
      "inputs": [
      ],
      "outputs": [
      ]
    },
    {
      "name": "NewVersion",
      "inputs": [
        {"name":"sid","type":"uint16"},
        {"components":[{"name":"major","type":"uint32"},{"name":"minor","type":"uint32"}],"name":"version","type":"tuple"},
        {"name":"hash","type":"uint256"},
        {"name":"initial","type":"bool"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "SetActivation",
      "inputs": [
        {"components":[{"name":"major","type":"uint32"},{"name":"minor","type":"uint32"}],"name":"version","type":"tuple"},
        {"name":"active","type":"bool"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "NftCreated",
      "inputs": [
        {"name":"id","type":"uint256"},
        {"name":"nft","type":"address"},
        {"name":"owner","type":"address"},
        {"name":"manager","type":"address"},
        {"name":"creator","type":"address"}
      ],
      "outputs": [
      ]
    },
    {
      "name": "NftBurned",
      "inputs": [
        {"name":"id","type":"uint256"},
        {"name":"nft","type":"address"},
        {"name":"owner","type":"address"},
        {"name":"manager","type":"address"}
      ],
      "outputs": [
      ]
    }
  ],
  "fields": [
    {"name":"_pubkey","type":"uint256"},
    {"name":"_timestamp","type":"uint64"},
    {"name":"_constructorFlag","type":"bool"},
    {"name":"_totalSupply","type":"uint128"},
    {"name":"_nftCode","type":"cell"},
    {"name":"_indexBasisCode","type":"cell"},
    {"name":"_indexCode","type":"cell"},
    {"name":"_json","type":"string"},
    {"name":"_token","type":"address"},
    {"name":"_wallet","type":"address"},
    {"name":"_balance","type":"uint128"},
    {"name":"_slaves","type":"optional(cell)"},
    {"name":"_randomNonce","type":"uint256"},
    {"name":"_tld","type":"string"},
    {"name":"_dao","type":"address"},
    {"name":"_admin","type":"address"},
    {"name":"_active","type":"bool"},
    {"components":[{"name":"maxNameLength","type":"uint32"},{"name":"maxPathLength","type":"uint32"},{"name":"minDuration","type":"uint32"},{"name":"maxDuration","type":"uint32"},{"name":"graceFinePercent","type":"uint128"},{"name":"startZeroAuctionFee","type":"uint128"}],"name":"_config","type":"tuple"},
    {"components":[{"name":"longPrice","type":"uint128"},{"name":"shortPrices","type":"uint128[]"},{"name":"onlyLettersFeePercent","type":"uint128"},{"name":"needZeroAuctionLength","type":"uint32"}],"name":"_priceConfig","type":"tuple"},
    {"components":[{"name":"auctionRoot","type":"address"},{"name":"tokenRoot","type":"address"},{"name":"duration","type":"uint32"}],"name":"_auctionConfig","type":"tuple"},
    {"components":[{"name":"startZeroAuction","type":"uint32"},{"name":"expiring","type":"uint32"},{"name":"grace","type":"uint32"}],"name":"_durationConfig","type":"tuple"}
  ]
}
"#;

pub const ONCHAIN_SITE_ABI_JSON: &'static str = r#"
{
	"ABI version": 2,
	"version": "2.3",
	"header": ["pubkey", "time", "expire"],
	"functions": [
		{
			"name": "constructor",
			"inputs": [
				{"name":"contentType","type":"optional(string)"}
			],
			"outputs": [
			]
		},
		{
			"name": "upload",
			"inputs": [
				{"name":"part","type":"cell"},
				{"name":"index","type":"uint8"}
			],
			"outputs": [
			]
		},
		{
			"name": "getDetails",
			"inputs": [
			],
			"outputs": [
				{"name":"content","type":"map(uint8,cell)"},
				{"name":"contentType","type":"string"}
			]
		},
		{
			"name": "destruct",
			"inputs": [
				{"name":"dest","type":"address"}
			],
			"outputs": [
			]
		},
		{
			"name": "transferOwnership",
			"inputs": [
				{"name":"newOwner","type":"uint256"}
			],
			"outputs": [
			]
		},
		{
			"name": "owner",
			"inputs": [
			],
			"outputs": [
				{"name":"owner","type":"uint256"}
			]
		},
		{
			"name": "_randomNonce",
			"inputs": [
			],
			"outputs": [
				{"name":"_randomNonce","type":"uint256"}
			]
		}
	],
	"data": [
		{"key":1,"name":"_randomNonce","type":"uint256"}
	],
	"events": [
		{
			"name": "OwnershipTransferred",
			"inputs": [
				{"name":"previousOwner","type":"uint256"},
				{"name":"newOwner","type":"uint256"}
			],
			"outputs": [
			]
		}
	],
	"fields": [
		{"name":"_pubkey","type":"uint256"},
		{"name":"_timestamp","type":"uint64"},
		{"name":"_constructorFlag","type":"bool"},
		{"name":"owner","type":"uint256"},
		{"name":"_randomNonce","type":"uint256"},
		{"name":"content","type":"map(uint8,cell)"},
		{"name":"_contentType","type":"string"}
	]
}

"#;
