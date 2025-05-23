CREATE TABLE
IF
  NOT EXISTS limitorders (
    limitOrderId VARCHAR(100) PRIMARY KEY NOT NULL
    , walletAddress VARCHAR(100)
    , buyTokenAddress VARCHAR(100)
    , sellTokenAddress VARCHAR(100)
    , sellTokenAmount NUMERIC
    , sellTokenDecimals INTEGER
    , tokenValue NUMERIC
    , sellType VARCHAR(100)
    , limitOrderType VARCHAR(100)
    , tokenAddressOfInterest VARCHAR(100)
    , orderStatus VARCHAR(100) DEFAULT 'open'
    , createdAt TIMESTAMP DEFAULT CURRENT_TIMESTAMP
  );