return {
	addPercentage = function(current: number, percentage:number):number
		return current + (current / 100 * percentage)
	end,
	subPercentage = function(current:number, percentage:number):number
		return current - (current / 100 * percentage)
	end
}