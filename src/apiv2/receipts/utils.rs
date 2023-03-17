use super::NewItemObject;

pub fn calc_subtotal(items: &Vec<NewItemObject>) -> f32 {
  let mut total = 0.0;

  for item in items {
    total += item.price - item.discount
  }
  total
}
