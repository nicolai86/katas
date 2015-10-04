package leap

func IsLeapYear(year int) bool {
	var isLeap = year%4 == 0
	var isCentury = year%100 == 0
	var isLeapCentury = year%400 == 0
	return (isLeap && !isCentury) || isLeapCentury
}
