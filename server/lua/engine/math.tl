return {
	addPercentage = function(current: integer, percentage:integer):integer
		return current + math.floor(current / 100 * percentage)
	end,
	subPercentage = function(current:integer, percentage:integer):integer
		return current - math.floor(current / 100 * percentage)
	end
}