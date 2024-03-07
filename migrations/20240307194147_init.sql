-- public.collections definition
CREATE TABLE public.collections (
	id serial4 NOT NULL,
	title text NOT NULL,
	CONSTRAINT collections_pk PRIMARY KEY (id)
);

-- public.codes definition
CREATE TABLE public.codes (
	code text NOT NULL,
	is_used bool NOT NULL,
	collection_id int4 NULL,
	CONSTRAINT codes_pk PRIMARY KEY (code),
	CONSTRAINT codes_fk FOREIGN KEY (collection_id) REFERENCES public.collections(id) ON DELETE SET NULL ON UPDATE CASCADE
);