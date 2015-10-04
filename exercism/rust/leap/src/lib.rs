pub fn is_leap_year(year: i32) -> bool {
  let is_leap = year % 4 == 0;
  let is_century = year % 100 == 0;
  let is_leap_century = year % 400 == 0;
  return (is_leap && !is_century) || is_leap_century;
}
