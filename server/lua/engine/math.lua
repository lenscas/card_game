return {
	addPercentage = function(current, percentage)
		return current + (current / 100 * percentage)
	end,
	subPercentage = function(current, percentage)
		return current - (current / 100 * percentage)
	end
}