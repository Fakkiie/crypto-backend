-- queries/users.sql

--! get_all_limitOrders
SELECT
  limitOrderId
  , walletAddress
  , buyTokenAddress
  , sellTokenAddress
  , sellTokenAmount
  , sellTokenDecimals
  , tokenValue
  , sellType
  , limitOrderType
  , tokenAddressOfInterest
  , orderStatus
  , createdAt
FROM
  limitorders;

--! insert_limitOrder
INSERT INTO
  limitorders (
    limitOrderId
    , walletAddress
    , buyTokenAddress
    , sellTokenAddress
    , sellTokenAmount
    , sellTokenDecimals
    , tokenValue
    , sellType
    , limitOrderType
    , tokenAddressOfInterest
    , orderStatus
  )
VALUES
  (
    :limitOrderId
    , :walletAddress
    , :buyTokenAddress
    , :sellTokenAddress
    , :sellTokenAmount
    , :sellTokenDecimals
    , :tokenValue
    , :sellType
    , :limitOrderType
    , :tokenAddressOfInterest
    , :orderStatus
  ) RETURNING limitOrderId
  , walletAddress
  , buyTokenAddress
  , sellTokenAddress
  , sellTokenAmount
  , sellTokenDecimals
  , tokenValue
  , sellType
  , limitOrderType
  , tokenAddressOfInterest
  , orderStatus
  , createdAt;

--! get_limitOrder
SELECT
  limitOrderId
  , walletAddress
  , buyTokenAddress
  , sellTokenAddress
  , sellTokenAmount
  , sellTokenDecimals
  , tokenValue
  , sellType
  , limitOrderType
  , tokenAddressOfInterest
  , orderStatus
  , createdAt
FROM
  limitorders
WHERE
  limitOrderId = :limitOrderId;

--! get_limitOrders_by_walletAddress
SELECT
  limitOrderId
  , walletAddress
  , buyTokenAddress
  , sellTokenAddress
  , sellTokenAmount
  , sellTokenDecimals
  , tokenValue
  , sellType
  , limitOrderType
  , tokenAddressOfInterest
  , orderStatus
  , createdAt
FROM
  limitorders
WHERE
  walletAddress = :walletAddress
  AND orderStatus = 'open';

--! delete_limitOrder
DELETE FROM
  limitorders
WHERE
  limitOrderId = :limitOrderId RETURNING limitOrderId
  , walletAddress
  , buyTokenAddress
  , sellTokenAddress
  , sellTokenAmount
  , sellTokenDecimals
  , tokenValue
  , sellType
  , limitOrderType
  , tokenAddressOfInterest
  , orderStatus
  , createdAt;

--! update_limitOrder
UPDATE
  limitorders
SET buyTokenAddress = :buyTokenAddress, sellTokenAddress = :sellTokenAddress, sellTokenAmount = :sellTokenAmount, tokenValue = :tokenValue, sellType = :sellType, limitOrderType = :limitOrderType, tokenAddressOfInterest = :tokenAddressOfInterest, orderStatus = :orderStatus
WHERE
  limitOrderId = :limitOrderId RETURNING limitOrderId
  , walletAddress
  , buyTokenAddress
  , sellTokenAddress
  , sellTokenAmount
  , sellTokenDecimals
  , tokenValue
  , sellType
  , limitOrderType
  , tokenAddressOfInterest
  , orderStatus
  , createdAt;

--! get_limitOrders_tokens_of_interest
SELECT DISTINCT
  tokenAddressOfInterest
FROM
  limitorders
WHERE
  orderStatus = 'open';

--! get_all_open_limitOrders
SELECT
  limitOrderId
  , walletAddress
  , buyTokenAddress
  , sellTokenAddress
  , sellTokenAmount
  , sellTokenDecimals
  , tokenValue
  , sellType
  , limitOrderType
  , tokenAddressOfInterest
  , orderStatus
  , createdAt
FROM
  limitorders
WHERE
  orderStatus = 'open';

--! close_limitOrder
UPDATE
  limitorders
SET orderStatus = 'closed'
WHERE
  limitOrderId = :limitOrderId RETURNING limitOrderId
  , walletAddress
  , buyTokenAddress
  , sellTokenAddress
  , sellTokenAmount
  , sellTokenDecimals
  , tokenValue
  , sellType
  , limitOrderType
  , tokenAddressOfInterest
  , orderStatus
  , createdAt;

--! pend_limitOrder
UPDATE
  limitorders
SET orderStatus = 'pending'
WHERE
  limitOrderId = :limitOrderId RETURNING limitOrderId
  , walletAddress
  , buyTokenAddress
  , sellTokenAddress
  , sellTokenAmount
  , sellTokenDecimals
  , tokenValue
  , sellType
  , limitOrderType
  , tokenAddressOfInterest
  , orderStatus
  , createdAt;

--! complete_limitOrder
UPDATE
  limitorders
SET orderStatus = 'complete'
WHERE
  limitOrderId = :limitOrderId RETURNING limitOrderId
  , walletAddress
  , buyTokenAddress
  , sellTokenAddress
  , sellTokenAmount
  , sellTokenDecimals
  , tokenValue
  , sellType
  , limitOrderType
  , tokenAddressOfInterest
  , orderStatus
  , createdAt;

--! error_limitOrder
UPDATE
  limitorders
SET orderStatus = 'error'
WHERE
  limitOrderId = :limitOrderId RETURNING limitOrderId
  , walletAddress
  , buyTokenAddress
  , sellTokenAddress
  , sellTokenAmount
  , sellTokenDecimals
  , tokenValue
  , sellType
  , limitOrderType
  , tokenAddressOfInterest
  , orderStatus
  , createdAt;