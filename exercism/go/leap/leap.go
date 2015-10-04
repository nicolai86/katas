/*
Package leap includes predicates useful when working with dates
*/
package leap

// IsLeapYear returns true if a given year is a leap year
func IsLeapYear(year int) bool {
	var isLeap = year%4 == 0
	var isCentury = year%100 == 0
	var isLeapCentury = year%400 == 0
	return (isLeap && !isCentury) || isLeapCentury
}
