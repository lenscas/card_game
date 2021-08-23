ALTER TABLE public.characters
   ADD COLUMN current_event json;

INSERT INTO characters (current_event)
    SELECT current_battle from characters;

ALTER TABLE public.characters
	DROP COLUMN current_battle;
