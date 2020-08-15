--This exposes some constants that are often used.

return {
	PATH_RAW_CARDS = "./cards",
	PATH_COMPILED_CARDS = "./compiled/cards",
	PATH_RAW_SMALL_RUNES = "./small_runes",
	PATH_RAW_HEXA_RUNES = "./hexa_runes",
	PATH_COMPILED_SMALL_RUNES = "./compiled/small_runes",
	PATH_COMPILED_HEXA_RUNES = "./compiled/hexa_runes",
	PATH_BASIC_CARD_FRAME = "../assets/basic_card_frame.svg",
	PATH_OUT_CARD_PICTURES = "../assets/cards",
	CARD_FRAME_REPLACEMENTS = {
		NAME = "{CARD_NAME}",
		DESCRIPTION = "{CARD_DESCRIPTION}",
		SPEED = "{SPEED}",
		MANA = "{MANA}"
	},
	PATH_ENV_FILE = "../.env"
}
