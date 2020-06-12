--
-- PostgreSQL database dump
--

-- Dumped from database version 11.7 (Debian 11.7-0+deb10u1)
-- Dumped by pg_dump version 11.7 (Debian 11.7-0+deb10u1)

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

SET default_with_oids = false;

--
-- Name: cards; Type: TABLE; Schema: public; Owner: admin
--

CREATE TABLE public.cards (
    id bigint NOT NULL,
    json_file_path text NOT NULL,
    text_id character varying(10) NOT NULL,
    is_obtainable boolean DEFAULT true NOT NULL
);


ALTER TABLE public.cards OWNER TO admin;

--
-- Name: cards_id_seq; Type: SEQUENCE; Schema: public; Owner: admin
--

CREATE SEQUENCE public.cards_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.cards_id_seq OWNER TO admin;

--
-- Name: cards_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: admin
--

ALTER SEQUENCE public.cards_id_seq OWNED BY public.cards.id;


--
-- Name: cards_in_deck; Type: TABLE; Schema: public; Owner: admin
--

CREATE TABLE public.cards_in_deck (
    card_id bigint NOT NULL,
    deck_id bigint NOT NULL
);


ALTER TABLE public.cards_in_deck OWNER TO admin;

--
-- Name: characters; Type: TABLE; Schema: public; Owner: admin
--

CREATE TABLE public.characters (
    id bigint NOT NULL,
    user_id bigint NOT NULL,
    current_battle text
);


ALTER TABLE public.characters OWNER TO admin;

--
-- Name: characters_id_seq; Type: SEQUENCE; Schema: public; Owner: admin
--

CREATE SEQUENCE public.characters_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.characters_id_seq OWNER TO admin;

--
-- Name: characters_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: admin
--

ALTER SEQUENCE public.characters_id_seq OWNED BY public.characters.id;


--
-- Name: decks; Type: TABLE; Schema: public; Owner: admin
--

CREATE TABLE public.decks (
    id bigint NOT NULL,
    character_id bigint NOT NULL
);


ALTER TABLE public.decks OWNER TO admin;

--
-- Name: decks_id_seq; Type: SEQUENCE; Schema: public; Owner: admin
--

CREATE SEQUENCE public.decks_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.decks_id_seq OWNER TO admin;

--
-- Name: decks_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: admin
--

ALTER SEQUENCE public.decks_id_seq OWNED BY public.decks.id;


--
-- Name: sessions; Type: TABLE; Schema: public; Owner: admin
--

CREATE TABLE public.sessions (
    hash text NOT NULL,
    user_id integer NOT NULL,
    activated_at timestamp with time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.sessions OWNER TO admin;

--
-- Name: users; Type: TABLE; Schema: public; Owner: admin
--

CREATE TABLE public.users (
    id integer NOT NULL,
    username text NOT NULL,
    password text NOT NULL
);


ALTER TABLE public.users OWNER TO admin;

--
-- Name: users_id_seq; Type: SEQUENCE; Schema: public; Owner: admin
--

CREATE SEQUENCE public.users_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.users_id_seq OWNER TO admin;

--
-- Name: users_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: admin
--

ALTER SEQUENCE public.users_id_seq OWNED BY public.users.id;


--
-- Name: cards id; Type: DEFAULT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.cards ALTER COLUMN id SET DEFAULT nextval('public.cards_id_seq'::regclass);


--
-- Name: characters id; Type: DEFAULT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.characters ALTER COLUMN id SET DEFAULT nextval('public.characters_id_seq'::regclass);


--
-- Name: decks id; Type: DEFAULT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.decks ALTER COLUMN id SET DEFAULT nextval('public.decks_id_seq'::regclass);


--
-- Name: users id; Type: DEFAULT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.users ALTER COLUMN id SET DEFAULT nextval('public.users_id_seq'::regclass);


--
-- Name: cards_in_deck cards_in_deck_pkey; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.cards_in_deck
    ADD CONSTRAINT cards_in_deck_pkey PRIMARY KEY (card_id, deck_id);


--
-- Name: cards cards_json_file_path_key; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.cards
    ADD CONSTRAINT cards_json_file_path_key UNIQUE (json_file_path);


--
-- Name: cards cards_pkey; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.cards
    ADD CONSTRAINT cards_pkey PRIMARY KEY (id);


--
-- Name: cards cards_text_id_key; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.cards
    ADD CONSTRAINT cards_text_id_key UNIQUE (text_id);


--
-- Name: characters characters_pkey; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.characters
    ADD CONSTRAINT characters_pkey PRIMARY KEY (id);


--
-- Name: decks decks_pkey; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.decks
    ADD CONSTRAINT decks_pkey PRIMARY KEY (id);


--
-- Name: sessions sessions_pkey; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.sessions
    ADD CONSTRAINT sessions_pkey PRIMARY KEY (hash);


--
-- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (id);


--
-- Name: users users_username_key; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_username_key UNIQUE (username);


--
-- Name: cards_in_deck cards_in_deck_card_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.cards_in_deck
    ADD CONSTRAINT cards_in_deck_card_id_fkey FOREIGN KEY (card_id) REFERENCES public.cards(id);


--
-- Name: characters characters_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.characters
    ADD CONSTRAINT characters_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id);


--
-- Name: decks decks_character_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.decks
    ADD CONSTRAINT decks_character_id_fkey FOREIGN KEY (character_id) REFERENCES public.characters(id);


--
-- Name: sessions sessions_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.sessions
    ADD CONSTRAINT sessions_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id);


--
-- PostgreSQL database dump complete
--

