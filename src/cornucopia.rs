// This file was generated with `cornucopia`. Do not modify.

#[allow(clippy::all, clippy::pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod types { }#[allow(clippy::all, clippy::pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod queries
{ pub mod limit_orders
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug)] pub struct InsertLimitOrderParams<T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,T3: cornucopia_async::StringSql,T4: cornucopia_async::StringSql,T5: cornucopia_async::StringSql,T6: cornucopia_async::StringSql,T7: cornucopia_async::StringSql,> { pub limitOrderId: T1,pub walletAddress: T2,pub buyTokenAddress: T3,pub sellTokenAddress: T4,pub sellTokenAmount: rust_decimal::Decimal,pub tokenValue: rust_decimal::Decimal,pub sellType: T5,pub limitOrderType: T6,pub tokenAddressOfInterest: T7,}#[derive( Debug)] pub struct UpdateLimitOrderParams<T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,T3: cornucopia_async::StringSql,T4: cornucopia_async::StringSql,T5: cornucopia_async::StringSql,T6: cornucopia_async::StringSql,> { pub buyTokenAddress: T1,pub sellTokenAddress: T2,pub sellTokenAmount: rust_decimal::Decimal,pub tokenValue: rust_decimal::Decimal,pub sellType: T3,pub limitOrderType: T4,pub tokenAddressOfInterest: T5,pub limitOrderId: T6,}#[derive( Debug, Clone, PartialEq,)] pub struct GetAllLimitOrders
{ pub limitorderid : String,pub walletaddress : String,pub buytokenaddress : String,pub selltokenaddress : String,pub selltokenamount : rust_decimal::Decimal,pub tokenvalue : rust_decimal::Decimal,pub selltype : String,pub limitordertype : String,pub tokenaddressofinterest : String,pub createdat : time::PrimitiveDateTime,}pub struct GetAllLimitOrdersBorrowed<'a> { pub limitorderid : &'a str,pub walletaddress : &'a str,pub buytokenaddress : &'a str,pub selltokenaddress : &'a str,pub selltokenamount : rust_decimal::Decimal,pub tokenvalue : rust_decimal::Decimal,pub selltype : &'a str,pub limitordertype : &'a str,pub tokenaddressofinterest : &'a str,pub createdat : time::PrimitiveDateTime,}
impl<'a> From<GetAllLimitOrdersBorrowed<'a>> for GetAllLimitOrders
{
    fn from(GetAllLimitOrdersBorrowed { limitorderid,walletaddress,buytokenaddress,selltokenaddress,selltokenamount,tokenvalue,selltype,limitordertype,tokenaddressofinterest,createdat,}: GetAllLimitOrdersBorrowed<'a>) ->
    Self { Self { limitorderid: limitorderid.into(),walletaddress: walletaddress.into(),buytokenaddress: buytokenaddress.into(),selltokenaddress: selltokenaddress.into(),selltokenamount,tokenvalue,selltype: selltype.into(),limitordertype: limitordertype.into(),tokenaddressofinterest: tokenaddressofinterest.into(),createdat,} }
}pub struct GetAllLimitOrdersQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> GetAllLimitOrdersBorrowed,
    mapper: fn(GetAllLimitOrdersBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> GetAllLimitOrdersQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(GetAllLimitOrdersBorrowed) -> R) ->
    GetAllLimitOrdersQuery<'a,C,R,N>
    {
        GetAllLimitOrdersQuery
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params) .await?
        .map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
}#[derive( Debug, Clone, PartialEq,)] pub struct InsertLimitOrder
{ pub limitorderid : String,pub walletaddress : String,pub buytokenaddress : String,pub selltokenaddress : String,pub selltokenamount : rust_decimal::Decimal,pub tokenvalue : rust_decimal::Decimal,pub selltype : String,pub limitordertype : String,pub tokenaddressofinterest : String,pub createdat : time::PrimitiveDateTime,}pub struct InsertLimitOrderBorrowed<'a> { pub limitorderid : &'a str,pub walletaddress : &'a str,pub buytokenaddress : &'a str,pub selltokenaddress : &'a str,pub selltokenamount : rust_decimal::Decimal,pub tokenvalue : rust_decimal::Decimal,pub selltype : &'a str,pub limitordertype : &'a str,pub tokenaddressofinterest : &'a str,pub createdat : time::PrimitiveDateTime,}
impl<'a> From<InsertLimitOrderBorrowed<'a>> for InsertLimitOrder
{
    fn from(InsertLimitOrderBorrowed { limitorderid,walletaddress,buytokenaddress,selltokenaddress,selltokenamount,tokenvalue,selltype,limitordertype,tokenaddressofinterest,createdat,}: InsertLimitOrderBorrowed<'a>) ->
    Self { Self { limitorderid: limitorderid.into(),walletaddress: walletaddress.into(),buytokenaddress: buytokenaddress.into(),selltokenaddress: selltokenaddress.into(),selltokenamount,tokenvalue,selltype: selltype.into(),limitordertype: limitordertype.into(),tokenaddressofinterest: tokenaddressofinterest.into(),createdat,} }
}pub struct InsertLimitOrderQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> InsertLimitOrderBorrowed,
    mapper: fn(InsertLimitOrderBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> InsertLimitOrderQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(InsertLimitOrderBorrowed) -> R) ->
    InsertLimitOrderQuery<'a,C,R,N>
    {
        InsertLimitOrderQuery
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params) .await?
        .map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
}#[derive( Debug, Clone, PartialEq,)] pub struct GetLimitOrder
{ pub limitorderid : String,pub walletaddress : String,pub buytokenaddress : String,pub selltokenaddress : String,pub selltokenamount : rust_decimal::Decimal,pub tokenvalue : rust_decimal::Decimal,pub selltype : String,pub limitordertype : String,pub tokenaddressofinterest : String,pub createdat : time::PrimitiveDateTime,}pub struct GetLimitOrderBorrowed<'a> { pub limitorderid : &'a str,pub walletaddress : &'a str,pub buytokenaddress : &'a str,pub selltokenaddress : &'a str,pub selltokenamount : rust_decimal::Decimal,pub tokenvalue : rust_decimal::Decimal,pub selltype : &'a str,pub limitordertype : &'a str,pub tokenaddressofinterest : &'a str,pub createdat : time::PrimitiveDateTime,}
impl<'a> From<GetLimitOrderBorrowed<'a>> for GetLimitOrder
{
    fn from(GetLimitOrderBorrowed { limitorderid,walletaddress,buytokenaddress,selltokenaddress,selltokenamount,tokenvalue,selltype,limitordertype,tokenaddressofinterest,createdat,}: GetLimitOrderBorrowed<'a>) ->
    Self { Self { limitorderid: limitorderid.into(),walletaddress: walletaddress.into(),buytokenaddress: buytokenaddress.into(),selltokenaddress: selltokenaddress.into(),selltokenamount,tokenvalue,selltype: selltype.into(),limitordertype: limitordertype.into(),tokenaddressofinterest: tokenaddressofinterest.into(),createdat,} }
}pub struct GetLimitOrderQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> GetLimitOrderBorrowed,
    mapper: fn(GetLimitOrderBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> GetLimitOrderQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(GetLimitOrderBorrowed) -> R) ->
    GetLimitOrderQuery<'a,C,R,N>
    {
        GetLimitOrderQuery
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params) .await?
        .map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
}#[derive( Debug, Clone, PartialEq,)] pub struct GetLimitOrdersByWalletAddress
{ pub limitorderid : String,pub walletaddress : String,pub buytokenaddress : String,pub selltokenaddress : String,pub selltokenamount : rust_decimal::Decimal,pub tokenvalue : rust_decimal::Decimal,pub selltype : String,pub limitordertype : String,pub tokenaddressofinterest : String,pub createdat : time::PrimitiveDateTime,}pub struct GetLimitOrdersByWalletAddressBorrowed<'a> { pub limitorderid : &'a str,pub walletaddress : &'a str,pub buytokenaddress : &'a str,pub selltokenaddress : &'a str,pub selltokenamount : rust_decimal::Decimal,pub tokenvalue : rust_decimal::Decimal,pub selltype : &'a str,pub limitordertype : &'a str,pub tokenaddressofinterest : &'a str,pub createdat : time::PrimitiveDateTime,}
impl<'a> From<GetLimitOrdersByWalletAddressBorrowed<'a>> for GetLimitOrdersByWalletAddress
{
    fn from(GetLimitOrdersByWalletAddressBorrowed { limitorderid,walletaddress,buytokenaddress,selltokenaddress,selltokenamount,tokenvalue,selltype,limitordertype,tokenaddressofinterest,createdat,}: GetLimitOrdersByWalletAddressBorrowed<'a>) ->
    Self { Self { limitorderid: limitorderid.into(),walletaddress: walletaddress.into(),buytokenaddress: buytokenaddress.into(),selltokenaddress: selltokenaddress.into(),selltokenamount,tokenvalue,selltype: selltype.into(),limitordertype: limitordertype.into(),tokenaddressofinterest: tokenaddressofinterest.into(),createdat,} }
}pub struct GetLimitOrdersByWalletAddressQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> GetLimitOrdersByWalletAddressBorrowed,
    mapper: fn(GetLimitOrdersByWalletAddressBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> GetLimitOrdersByWalletAddressQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(GetLimitOrdersByWalletAddressBorrowed) -> R) ->
    GetLimitOrdersByWalletAddressQuery<'a,C,R,N>
    {
        GetLimitOrdersByWalletAddressQuery
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params) .await?
        .map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
}#[derive( Debug, Clone, PartialEq,)] pub struct DeleteLimitOrder
{ pub limitorderid : String,pub walletaddress : String,pub buytokenaddress : String,pub selltokenaddress : String,pub selltokenamount : rust_decimal::Decimal,pub tokenvalue : rust_decimal::Decimal,pub selltype : String,pub limitordertype : String,pub tokenaddressofinterest : String,pub createdat : time::PrimitiveDateTime,}pub struct DeleteLimitOrderBorrowed<'a> { pub limitorderid : &'a str,pub walletaddress : &'a str,pub buytokenaddress : &'a str,pub selltokenaddress : &'a str,pub selltokenamount : rust_decimal::Decimal,pub tokenvalue : rust_decimal::Decimal,pub selltype : &'a str,pub limitordertype : &'a str,pub tokenaddressofinterest : &'a str,pub createdat : time::PrimitiveDateTime,}
impl<'a> From<DeleteLimitOrderBorrowed<'a>> for DeleteLimitOrder
{
    fn from(DeleteLimitOrderBorrowed { limitorderid,walletaddress,buytokenaddress,selltokenaddress,selltokenamount,tokenvalue,selltype,limitordertype,tokenaddressofinterest,createdat,}: DeleteLimitOrderBorrowed<'a>) ->
    Self { Self { limitorderid: limitorderid.into(),walletaddress: walletaddress.into(),buytokenaddress: buytokenaddress.into(),selltokenaddress: selltokenaddress.into(),selltokenamount,tokenvalue,selltype: selltype.into(),limitordertype: limitordertype.into(),tokenaddressofinterest: tokenaddressofinterest.into(),createdat,} }
}pub struct DeleteLimitOrderQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> DeleteLimitOrderBorrowed,
    mapper: fn(DeleteLimitOrderBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> DeleteLimitOrderQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(DeleteLimitOrderBorrowed) -> R) ->
    DeleteLimitOrderQuery<'a,C,R,N>
    {
        DeleteLimitOrderQuery
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params) .await?
        .map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
}#[derive( Debug, Clone, PartialEq,)] pub struct UpdateLimitOrder
{ pub limitorderid : String,pub walletaddress : String,pub buytokenaddress : String,pub selltokenaddress : String,pub selltokenamount : rust_decimal::Decimal,pub tokenvalue : rust_decimal::Decimal,pub selltype : String,pub limitordertype : String,pub tokenaddressofinterest : String,pub createdat : time::PrimitiveDateTime,}pub struct UpdateLimitOrderBorrowed<'a> { pub limitorderid : &'a str,pub walletaddress : &'a str,pub buytokenaddress : &'a str,pub selltokenaddress : &'a str,pub selltokenamount : rust_decimal::Decimal,pub tokenvalue : rust_decimal::Decimal,pub selltype : &'a str,pub limitordertype : &'a str,pub tokenaddressofinterest : &'a str,pub createdat : time::PrimitiveDateTime,}
impl<'a> From<UpdateLimitOrderBorrowed<'a>> for UpdateLimitOrder
{
    fn from(UpdateLimitOrderBorrowed { limitorderid,walletaddress,buytokenaddress,selltokenaddress,selltokenamount,tokenvalue,selltype,limitordertype,tokenaddressofinterest,createdat,}: UpdateLimitOrderBorrowed<'a>) ->
    Self { Self { limitorderid: limitorderid.into(),walletaddress: walletaddress.into(),buytokenaddress: buytokenaddress.into(),selltokenaddress: selltokenaddress.into(),selltokenamount,tokenvalue,selltype: selltype.into(),limitordertype: limitordertype.into(),tokenaddressofinterest: tokenaddressofinterest.into(),createdat,} }
}pub struct UpdateLimitOrderQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> UpdateLimitOrderBorrowed,
    mapper: fn(UpdateLimitOrderBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> UpdateLimitOrderQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(UpdateLimitOrderBorrowed) -> R) ->
    UpdateLimitOrderQuery<'a,C,R,N>
    {
        UpdateLimitOrderQuery
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params) .await?
        .map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
}pub fn get_all_limitOrders() -> GetAllLimitOrdersStmt
{ GetAllLimitOrdersStmt(cornucopia_async::private::Stmt::new("SELECT 
    limitOrderId, 
    walletAddress, 
    buyTokenAddress, 
    sellTokenAddress, 
    sellTokenAmount, 
    tokenValue, 
    sellType, 
    limitOrderType,
    tokenAddressOfInterest,
    createdAt 
FROM limitorders")) } pub struct
GetAllLimitOrdersStmt(cornucopia_async::private::Stmt); impl GetAllLimitOrdersStmt
{ pub fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
) -> GetAllLimitOrdersQuery<'a,C,
GetAllLimitOrders, 0>
{
    GetAllLimitOrdersQuery
    {
        client, params: [], stmt: &mut self.0, extractor:
        |row| { GetAllLimitOrdersBorrowed { limitorderid: row.get(0),walletaddress: row.get(1),buytokenaddress: row.get(2),selltokenaddress: row.get(3),selltokenamount: row.get(4),tokenvalue: row.get(5),selltype: row.get(6),limitordertype: row.get(7),tokenaddressofinterest: row.get(8),createdat: row.get(9),} }, mapper: |it| { <GetAllLimitOrders>::from(it) },
    }
} }pub fn insert_limitOrder() -> InsertLimitOrderStmt
{ InsertLimitOrderStmt(cornucopia_async::private::Stmt::new("INSERT INTO limitorders (
    limitOrderId,
    walletAddress, 
    buyTokenAddress, 
    sellTokenAddress, 
    sellTokenAmount, 
    tokenValue, 
    sellType, 
    limitOrderType,
    tokenAddressOfInterest
) 
VALUES (
    $1,
    $2, 
    $3, 
    $4, 
    $5, 
    $6, 
    $7, 
    $8,
    $9
) 
RETURNING 
    limitOrderId, 
    walletAddress, 
    buyTokenAddress, 
    sellTokenAddress, 
    sellTokenAmount, 
    tokenValue, 
    sellType, 
    limitOrderType,
    tokenAddressOfInterest,
    createdAt")) } pub struct
InsertLimitOrderStmt(cornucopia_async::private::Stmt); impl InsertLimitOrderStmt
{ pub fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,T2:
cornucopia_async::StringSql,T3:
cornucopia_async::StringSql,T4:
cornucopia_async::StringSql,T5:
cornucopia_async::StringSql,T6:
cornucopia_async::StringSql,T7:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
limitOrderId: &'a T1,walletAddress: &'a T2,buyTokenAddress: &'a T3,sellTokenAddress: &'a T4,sellTokenAmount: &'a rust_decimal::Decimal,tokenValue: &'a rust_decimal::Decimal,sellType: &'a T5,limitOrderType: &'a T6,tokenAddressOfInterest: &'a T7,) -> InsertLimitOrderQuery<'a,C,
InsertLimitOrder, 9>
{
    InsertLimitOrderQuery
    {
        client, params: [limitOrderId,walletAddress,buyTokenAddress,sellTokenAddress,sellTokenAmount,tokenValue,sellType,limitOrderType,tokenAddressOfInterest,], stmt: &mut self.0, extractor:
        |row| { InsertLimitOrderBorrowed { limitorderid: row.get(0),walletaddress: row.get(1),buytokenaddress: row.get(2),selltokenaddress: row.get(3),selltokenamount: row.get(4),tokenvalue: row.get(5),selltype: row.get(6),limitordertype: row.get(7),tokenaddressofinterest: row.get(8),createdat: row.get(9),} }, mapper: |it| { <InsertLimitOrder>::from(it) },
    }
} }impl <'a, C: GenericClient,T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,T3: cornucopia_async::StringSql,T4: cornucopia_async::StringSql,T5: cornucopia_async::StringSql,T6: cornucopia_async::StringSql,T7: cornucopia_async::StringSql,> cornucopia_async::Params<'a,
InsertLimitOrderParams<T1,T2,T3,T4,T5,T6,T7,>, InsertLimitOrderQuery<'a, C,
InsertLimitOrder, 9>, C> for InsertLimitOrderStmt
{
    fn
    params(&'a mut self, client: &'a  C, params: &'a
    InsertLimitOrderParams<T1,T2,T3,T4,T5,T6,T7,>) -> InsertLimitOrderQuery<'a, C,
    InsertLimitOrder, 9>
    { self.bind(client, &params.limitOrderId,&params.walletAddress,&params.buyTokenAddress,&params.sellTokenAddress,&params.sellTokenAmount,&params.tokenValue,&params.sellType,&params.limitOrderType,&params.tokenAddressOfInterest,) }
}pub fn get_limitOrder() -> GetLimitOrderStmt
{ GetLimitOrderStmt(cornucopia_async::private::Stmt::new("SELECT 
    limitOrderId, 
    walletAddress, 
    buyTokenAddress, 
    sellTokenAddress, 
    sellTokenAmount, 
    tokenValue, 
    sellType, 
    limitOrderType,
    tokenAddressOfInterest,
    createdAt 
FROM limitorders 
WHERE limitOrderId = $1")) } pub struct
GetLimitOrderStmt(cornucopia_async::private::Stmt); impl GetLimitOrderStmt
{ pub fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
limitOrderId: &'a T1,) -> GetLimitOrderQuery<'a,C,
GetLimitOrder, 1>
{
    GetLimitOrderQuery
    {
        client, params: [limitOrderId,], stmt: &mut self.0, extractor:
        |row| { GetLimitOrderBorrowed { limitorderid: row.get(0),walletaddress: row.get(1),buytokenaddress: row.get(2),selltokenaddress: row.get(3),selltokenamount: row.get(4),tokenvalue: row.get(5),selltype: row.get(6),limitordertype: row.get(7),tokenaddressofinterest: row.get(8),createdat: row.get(9),} }, mapper: |it| { <GetLimitOrder>::from(it) },
    }
} }pub fn get_limitOrders_by_walletAddress() -> GetLimitOrdersByWalletAddressStmt
{ GetLimitOrdersByWalletAddressStmt(cornucopia_async::private::Stmt::new("SELECT 
    limitOrderId, 
    walletAddress, 
    buyTokenAddress, 
    sellTokenAddress, 
    sellTokenAmount, 
    tokenValue, 
    sellType, 
    limitOrderType,
    tokenAddressOfInterest,
    createdAt 
FROM limitorders 
WHERE walletAddress = $1")) } pub struct
GetLimitOrdersByWalletAddressStmt(cornucopia_async::private::Stmt); impl GetLimitOrdersByWalletAddressStmt
{ pub fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
walletAddress: &'a T1,) -> GetLimitOrdersByWalletAddressQuery<'a,C,
GetLimitOrdersByWalletAddress, 1>
{
    GetLimitOrdersByWalletAddressQuery
    {
        client, params: [walletAddress,], stmt: &mut self.0, extractor:
        |row| { GetLimitOrdersByWalletAddressBorrowed { limitorderid: row.get(0),walletaddress: row.get(1),buytokenaddress: row.get(2),selltokenaddress: row.get(3),selltokenamount: row.get(4),tokenvalue: row.get(5),selltype: row.get(6),limitordertype: row.get(7),tokenaddressofinterest: row.get(8),createdat: row.get(9),} }, mapper: |it| { <GetLimitOrdersByWalletAddress>::from(it) },
    }
} }pub fn delete_limitOrder() -> DeleteLimitOrderStmt
{ DeleteLimitOrderStmt(cornucopia_async::private::Stmt::new("DELETE FROM limitorders 
WHERE limitOrderId = $1 
RETURNING 
    limitOrderId, 
    walletAddress, 
    buyTokenAddress, 
    sellTokenAddress, 
    sellTokenAmount, 
    tokenValue, 
    sellType, 
    limitOrderType,
    tokenAddressOfInterest,
    createdAt")) } pub struct
DeleteLimitOrderStmt(cornucopia_async::private::Stmt); impl DeleteLimitOrderStmt
{ pub fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
limitOrderId: &'a T1,) -> DeleteLimitOrderQuery<'a,C,
DeleteLimitOrder, 1>
{
    DeleteLimitOrderQuery
    {
        client, params: [limitOrderId,], stmt: &mut self.0, extractor:
        |row| { DeleteLimitOrderBorrowed { limitorderid: row.get(0),walletaddress: row.get(1),buytokenaddress: row.get(2),selltokenaddress: row.get(3),selltokenamount: row.get(4),tokenvalue: row.get(5),selltype: row.get(6),limitordertype: row.get(7),tokenaddressofinterest: row.get(8),createdat: row.get(9),} }, mapper: |it| { <DeleteLimitOrder>::from(it) },
    }
} }pub fn update_limitOrder() -> UpdateLimitOrderStmt
{ UpdateLimitOrderStmt(cornucopia_async::private::Stmt::new("UPDATE limitorders 
SET 
    buyTokenAddress = $1, 
    sellTokenAddress = $2, 
    sellTokenAmount = $3, 
    tokenValue = $4, 
    sellType = $5, 
    limitOrderType = $6,
    tokenAddressOfInterest = $7 
WHERE limitOrderId = $8
RETURNING 
    limitOrderId, 
    walletAddress, 
    buyTokenAddress, 
    sellTokenAddress, 
    sellTokenAmount, 
    tokenValue, 
    sellType, 
    limitOrderType,
    tokenAddressOfInterest,
    createdAt")) } pub struct
UpdateLimitOrderStmt(cornucopia_async::private::Stmt); impl UpdateLimitOrderStmt
{ pub fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,T2:
cornucopia_async::StringSql,T3:
cornucopia_async::StringSql,T4:
cornucopia_async::StringSql,T5:
cornucopia_async::StringSql,T6:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
buyTokenAddress: &'a T1,sellTokenAddress: &'a T2,sellTokenAmount: &'a rust_decimal::Decimal,tokenValue: &'a rust_decimal::Decimal,sellType: &'a T3,limitOrderType: &'a T4,tokenAddressOfInterest: &'a T5,limitOrderId: &'a T6,) -> UpdateLimitOrderQuery<'a,C,
UpdateLimitOrder, 8>
{
    UpdateLimitOrderQuery
    {
        client, params: [buyTokenAddress,sellTokenAddress,sellTokenAmount,tokenValue,sellType,limitOrderType,tokenAddressOfInterest,limitOrderId,], stmt: &mut self.0, extractor:
        |row| { UpdateLimitOrderBorrowed { limitorderid: row.get(0),walletaddress: row.get(1),buytokenaddress: row.get(2),selltokenaddress: row.get(3),selltokenamount: row.get(4),tokenvalue: row.get(5),selltype: row.get(6),limitordertype: row.get(7),tokenaddressofinterest: row.get(8),createdat: row.get(9),} }, mapper: |it| { <UpdateLimitOrder>::from(it) },
    }
} }impl <'a, C: GenericClient,T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,T3: cornucopia_async::StringSql,T4: cornucopia_async::StringSql,T5: cornucopia_async::StringSql,T6: cornucopia_async::StringSql,> cornucopia_async::Params<'a,
UpdateLimitOrderParams<T1,T2,T3,T4,T5,T6,>, UpdateLimitOrderQuery<'a, C,
UpdateLimitOrder, 8>, C> for UpdateLimitOrderStmt
{
    fn
    params(&'a mut self, client: &'a  C, params: &'a
    UpdateLimitOrderParams<T1,T2,T3,T4,T5,T6,>) -> UpdateLimitOrderQuery<'a, C,
    UpdateLimitOrder, 8>
    { self.bind(client, &params.buyTokenAddress,&params.sellTokenAddress,&params.sellTokenAmount,&params.tokenValue,&params.sellType,&params.limitOrderType,&params.tokenAddressOfInterest,&params.limitOrderId,) }
}}}