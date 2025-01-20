-- queries/users.sql

--! get_all_limitOrders
SELECT
  limitOrderId
  , walletAddress
  , buyTokenAddress
  , sellTokenAddress
  , sellTokenAmount
  , tokenValue
  , sellType
  , limitOrderType
  , tokenAddressOfInterest
  , createdAt
FROM
  limitOrders;

--! insert_limitOrder
INSERT INTO
  limitOrders (
    limitOrderId
    , walletAddress
    , buyTokenAddress
    , sellTokenAddress
    , sellTokenAmount
    , tokenValue
    , sellType
    , limitOrderType
    , tokenAddressOfInterest
  )
VALUES
  (
    :limitOrderId
    , :walletAddress
    , :buyTokenAddress
    , :sellTokenAddress
    , :sellTokenAmount
    , :tokenValue
    , :sellType
    , :limitOrderType
    , :tokenAddressOfInterest
  ) RETURNING walletAddress
  , buyTokenAddress
  , sellTokenAddress
  , sellTokenAmount
  , tokenValue
  , sellType
  , limitOrderType
  , tokenAddressOfInterest
  , createdAt;

--! get_limitOrder
SELECT
  limitOrderId
  , walletAddress
  , buyTokenAddress
  , sellTokenAddress
  , sellTokenAmount
  , tokenValue
  , sellType
  , limitOrderType
  , tokenAddressOfInterest
  , createdAt
FROM
  limitOrders
WHERE
  limitOrderId = :limitOrderId;

--! get_limitOrders_by_walletAddress
SELECT
  limitOrderId
  , walletAddress
  , buyTokenAddress
  , sellTokenAddress
  , sellTokenAmount
  , tokenValue
  , sellType
  , limitOrderType
  , tokenAddressOfInterest
  , createdAt
FROM
  limitOrders
WHERE
  walletAddress = :walletAddress;

--! delete_limitOrder
DELETE FROM
  limitOrders
WHERE
  limitOrderId = :limitOrderId RETURNING limitOrderId
  , walletAddress
  , buyTokenAddress
  , sellTokenAddress
  , sellTokenAmount
  , tokenValue
  , sellType
  , limitOrderType
  , tokenAddressOfInterest
  , createdAt;

--! update_limitOrder
UPDATE
  limitOrders
SET buyTokenAddress = :buyTokenAddress, sellTokenAddress = :sellTokenAddress, sellTokenAmount = :sellTokenAmount, tokenValue = :tokenValue, sellType = :sellType, limitOrderType = :limitOrderType
WHERE
  limitOrderId = :limitOrderId RETURNING limitOrderId
  , walletAddress
  , buyTokenAddress
  , sellTokenAddress
  , sellTokenAmount
  , tokenValue
  , sellType
  , limitOrderType
  , tokenAddressOfInterest
  , createdAt;