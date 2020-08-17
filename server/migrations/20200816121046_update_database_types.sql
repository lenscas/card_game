ALTER TABLE public."characters" ALTER COLUMN current_battle TYPE _json USING current_battle::_json;
DELETE TABLE public.sessions;
CREATE TABLE public.sessions (
    hash char(44) NOT NULL,
    user_id integer NOT NULL,
    activated_at timestamp with time zone DEFAULT now() NOT NULL
);

DROP TABLE public.users;
CREATE TABLE public.users (
    id integer NOT NULL,
    username varchar(255) NOT NULL,
    password varchar(255) NOT NULL
);

CREATE SEQUENCE public.users_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;



ALTER SEQUENCE public.users_id_seq OWNED BY public.users.id;

ALTER TABLE ONLY public.users ALTER COLUMN id SET DEFAULT nextval('public.users_id_seq'::regclass);
ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (id);

ALTER TABLE ONLY public.sessions
    ADD CONSTRAINT sessions_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id);
ALTER TABLE ONLY public.sessions
    ADD CONSTRAINT sessions_pkey PRIMARY KEY (hash);