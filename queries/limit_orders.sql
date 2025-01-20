-- queries/users.sql

--! get_all_limitOrders
SELECT 
    limitOrderId, 
    walletAddress, 
    buyTokenAddress, 
    sellTokenAddress, 
    sellTokenAmount, 
    tokenValue, 
    sellType, 
    limitOrderType 
FROM limitorders;

--! insert_limitOrder
INSERT INTO limitorders (
    limitOrderId,
    walletAddress, 
    buyTokenAddress, 
    sellTokenAddress, 
    sellTokenAmount, 
    tokenValue, 
    sellType, 
    limitOrderType
) 
VALUES (
    :limitOrderId,
    :walletAddress, 
    :buyTokenAddress, 
    :sellTokenAddress, 
    :sellTokenAmount, 
    :tokenValue, 
    :sellType, 
    :limitOrderType
) 
RETURNING 
    limitOrderId, 
    walletAddress, 
    buyTokenAddress, 
    sellTokenAddress, 
    sellTokenAmount, 
    tokenValue, 
    sellType, 
    limitOrderType;

--! get_limitOrder
SELECT 
    limitOrderId, 
    walletAddress, 
    buyTokenAddress, 
    sellTokenAddress, 
    sellTokenAmount, 
    tokenValue, 
    sellType, 
    limitOrderType 
FROM limitorders 
WHERE limitOrderId = :limitOrderId;

--! get_limitOrders_by_walletAddress
SELECT 
    limitOrderId, 
    walletAddress, 
    buyTokenAddress, 
    sellTokenAddress, 
    sellTokenAmount, 
    tokenValue, 
    sellType, 
    limitOrderType 
FROM limitorders 
WHERE walletAddress = :walletAddress;

--! delete_limitOrder
DELETE FROM limitorders 
WHERE limitOrderId = :limitOrderId 
RETURNING 
    limitOrderId, 
    walletAddress, 
    buyTokenAddress, 
    sellTokenAddress, 
    sellTokenAmount, 
    tokenValue, 
    sellType, 
    limitOrderType;

--! update_limitOrder
UPDATE limitorders 
SET 
    buyTokenAddress = :buyTokenAddress, 
    sellTokenAddress = :sellTokenAddress, 
    sellTokenAmount = :sellTokenAmount, 
    tokenValue = :tokenValue, 
    sellType = :sellType, 
    limitOrderType = :limitOrderType 
WHERE limitOrderId = :limitOrderId
RETURNING 
    limitOrderId, 
    walletAddress, 
    buyTokenAddress, 
    sellTokenAddress, 
    sellTokenAmount, 
    tokenValue, 
    sellType, 
    limitOrderType;
