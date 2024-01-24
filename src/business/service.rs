
pub async fn get_business_state_by_business_id(
    business_id: web::Path<String>,
    data: web::Data<BusinessState>,
) -> Result<HttpResponse, Error> {
    let business_id = business_id.into_inner();
    let business = data
        .businesses
        .get(&business_id)
        .ok_or_else(|| HttpResponse::NotFound().finish())?;
    Ok(HttpResponse::Ok().json(business))
}