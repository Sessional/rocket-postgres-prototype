-- Table: public.stuff

-- DROP TABLE IF EXISTS public.stuff;

CREATE TABLE IF NOT EXISTS public.stuff
(
    id integer NOT NULL,
    name text COLLATE pg_catalog."default",
    CONSTRAINT stuff_pkey PRIMARY KEY (id)
);

INSERT INTO public.stuff (id, name) VALUES (1, 'do_stuff');
