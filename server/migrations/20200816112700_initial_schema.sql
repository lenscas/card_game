SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- TOC entry 202 (class 1259 OID 16493)
-- Name: cards; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.cards (
    id bigint NOT NULL,
    json_file_path text NOT NULL,
    text_id character varying(10) NOT NULL,
    is_obtainable boolean DEFAULT true NOT NULL,
    is_starting_card boolean NOT NULL
);


--
-- TOC entry 203 (class 1259 OID 16500)
-- Name: cards_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.cards_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- TOC entry 3001 (class 0 OID 0)
-- Dependencies: 203
-- Name: cards_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.cards_id_seq OWNED BY public.cards.id;


--
-- TOC entry 204 (class 1259 OID 16502)
-- Name: cards_in_deck; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.cards_in_deck (
    card_id bigint NOT NULL,
    deck_id bigint NOT NULL
);


--
-- TOC entry 205 (class 1259 OID 16505)
-- Name: characters; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.characters (
    id bigint NOT NULL,
    user_id bigint NOT NULL,
    current_battle json,
    dungeon json NOT NULL,
    character_state json NOT NULL
);


--
-- TOC entry 206 (class 1259 OID 16511)
-- Name: characters_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.characters_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- TOC entry 3002 (class 0 OID 0)
-- Dependencies: 206
-- Name: characters_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.characters_id_seq OWNED BY public.characters.id;


--
-- TOC entry 207 (class 1259 OID 16513)
-- Name: decks; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.decks (
    id bigint NOT NULL,
    character_id bigint NOT NULL
);


--
-- TOC entry 208 (class 1259 OID 16516)
-- Name: decks_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.decks_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- TOC entry 3003 (class 0 OID 0)
-- Dependencies: 208
-- Name: decks_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.decks_id_seq OWNED BY public.decks.id;


--
-- TOC entry 213 (class 1259 OID 16577)
-- Name: owned_starting_cards; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.owned_starting_cards (
    user_id bigint NOT NULL,
    id bigint NOT NULL,
    card_id bigint NOT NULL
);


--
-- TOC entry 212 (class 1259 OID 16575)
-- Name: owned_starting_cards_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.owned_starting_cards_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- TOC entry 3004 (class 0 OID 0)
-- Dependencies: 212
-- Name: owned_starting_cards_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.owned_starting_cards_id_seq OWNED BY public.owned_starting_cards.id;


--
-- TOC entry 209 (class 1259 OID 16518)
-- Name: sessions; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.sessions (
    hash char(44) NOT NULL,
    user_id bigint NOT NULL,
    activated_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- TOC entry 210 (class 1259 OID 16525)
-- Name: users; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.users (
    id bigint NOT NULL,
    username varchar(255) NOT NULL,
    password varchar(255) NOT NULL
);


--
-- TOC entry 211 (class 1259 OID 16531)
-- Name: users_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.users_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- TOC entry 3005 (class 0 OID 0)
-- Dependencies: 211
-- Name: users_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.users_id_seq OWNED BY public.users.id;


--
-- TOC entry 2834 (class 2604 OID 16533)
-- Name: cards id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cards ALTER COLUMN id SET DEFAULT nextval('public.cards_id_seq'::regclass);


--
-- TOC entry 2835 (class 2604 OID 16534)
-- Name: characters id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.characters ALTER COLUMN id SET DEFAULT nextval('public.characters_id_seq'::regclass);


--
-- TOC entry 2836 (class 2604 OID 16535)
-- Name: decks id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.decks ALTER COLUMN id SET DEFAULT nextval('public.decks_id_seq'::regclass);


--
-- TOC entry 2839 (class 2604 OID 16580)
-- Name: owned_starting_cards id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.owned_starting_cards ALTER COLUMN id SET DEFAULT nextval('public.owned_starting_cards_id_seq'::regclass);


--
-- TOC entry 2838 (class 2604 OID 16536)
-- Name: users id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.users ALTER COLUMN id SET DEFAULT nextval('public.users_id_seq'::regclass);


--
-- TOC entry 2848 (class 2606 OID 16538)
-- Name: cards_in_deck cards_in_deck_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cards_in_deck
    ADD CONSTRAINT cards_in_deck_pkey PRIMARY KEY (card_id, deck_id);


--
-- TOC entry 2841 (class 2606 OID 16540)
-- Name: cards cards_json_file_path_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cards
    ADD CONSTRAINT cards_json_file_path_key UNIQUE (json_file_path);


--
-- TOC entry 2843 (class 2606 OID 16542)
-- Name: cards cards_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cards
    ADD CONSTRAINT cards_pkey PRIMARY KEY (id);


--
-- TOC entry 2845 (class 2606 OID 16544)
-- Name: cards cards_text_id_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cards
    ADD CONSTRAINT cards_text_id_key UNIQUE (text_id);


--
-- TOC entry 2850 (class 2606 OID 16546)
-- Name: characters characters_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.characters
    ADD CONSTRAINT characters_pkey PRIMARY KEY (id);


--
-- TOC entry 2852 (class 2606 OID 16548)
-- Name: decks decks_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.decks
    ADD CONSTRAINT decks_pkey PRIMARY KEY (id);


--
-- TOC entry 2860 (class 2606 OID 16582)
-- Name: owned_starting_cards owned_starting_cards_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.owned_starting_cards
    ADD CONSTRAINT owned_starting_cards_pkey PRIMARY KEY (id);


--
-- TOC entry 2862 (class 2606 OID 16599)
-- Name: owned_starting_cards owned_starting_cards_user_id_card_id_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.owned_starting_cards
    ADD CONSTRAINT owned_starting_cards_user_id_card_id_key UNIQUE (user_id, card_id);


--
-- TOC entry 2854 (class 2606 OID 16550)
-- Name: sessions sessions_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.sessions
    ADD CONSTRAINT sessions_pkey PRIMARY KEY (hash);


--
-- TOC entry 2856 (class 2606 OID 16552)
-- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (id);


--
-- TOC entry 2858 (class 2606 OID 16554)
-- Name: users users_username_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_username_key UNIQUE (username);


--
-- TOC entry 2846 (class 1259 OID 16593)
-- Name: index_is_obtainable; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX index_is_obtainable ON public.cards USING btree (is_obtainable);


--
-- TOC entry 2863 (class 2606 OID 16555)
-- Name: cards_in_deck cards_in_deck_card_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.cards_in_deck
    ADD CONSTRAINT cards_in_deck_card_id_fkey FOREIGN KEY (card_id) REFERENCES public.cards(id);


--
-- TOC entry 2864 (class 2606 OID 16560)
-- Name: characters characters_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.characters
    ADD CONSTRAINT characters_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id);


--
-- TOC entry 2865 (class 2606 OID 16565)
-- Name: decks decks_character_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.decks
    ADD CONSTRAINT decks_character_id_fkey FOREIGN KEY (character_id) REFERENCES public.characters(id);


--
-- TOC entry 2867 (class 2606 OID 16583)
-- Name: owned_starting_cards owned_starting_cards_card_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.owned_starting_cards
    ADD CONSTRAINT owned_starting_cards_card_id_fkey FOREIGN KEY (card_id) REFERENCES public.cards(id);


--
-- TOC entry 2868 (class 2606 OID 16588)
-- Name: owned_starting_cards owned_starting_cards_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.owned_starting_cards
    ADD CONSTRAINT owned_starting_cards_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id);


--
-- TOC entry 2866 (class 2606 OID 16570)
-- Name: sessions sessions_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.sessions
    ADD CONSTRAINT sessions_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id);


-- Completed on 2020-06-28 01:01:11 CEST

--
-- PostgreSQL database dump complete
--

