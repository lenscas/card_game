--This exposes some constants that are often used.

return {
	PATH_RAW_CARDS = "./lua/cards",
	PATH_COMPILED_CARDS = "./lua/compiled/cards",
	PATH_RAW_SMALL_RUNES = "./lua/small_runes",
	PATH_RAW_HEXA_RUNES = "./lua/hexa_runes",
	PATH_COMPILED_SMALL_RUNES = "./lua/compiled/small_runes",
	PATH_COMPILED_HEXA_RUNES = "./lua/compiled/hexa_runes",
	PATH_BASIC_CARD_FRAME = "./assets/basic_card_frame.svg",
	PATH_OUT_CARD_PICTURES = "./assets/cards",
	FULL_PATH_CARD_GENERATION_TIME_ID_FILE = "./assets/cards/generation_time.txt",
	CARD_FRAME_REPLACEMENTS = {
		NAME = "{CARD_NAME}",
		DESCRIPTION = "{CARD_DESCRIPTION}",
		SPEED = "{SPEED}",
		MANA = "{MANA}"
	},
	PATH_ENV_FILE = "./.env"
}
