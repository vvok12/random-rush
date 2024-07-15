CREATE TABLE IF NOT EXISTS public.users
(
    id uuid NOT NULL,
    CONSTRAINT users_pkey PRIMARY KEY (id)
)

CREATE SEQUENCE IF NOT EXISTS public.board_deck_id_seq
    INCREMENT 1
    START 1
    MINVALUE 1
    MAXVALUE 9223372036854775807;

CREATE SEQUENCE IF NOT EXISTS public.board_graveyard_id_seq
    INCREMENT 1
    START 1
    MINVALUE 1
    MAXVALUE 9223372036854775807;

CREATE TABLE IF NOT EXISTS public.boards
(
    id uuid NOT NULL,
    deck_id bigint NOT NULL DEFAULT nextval('board_deck_id_seq'::regclass),
    graveyard_id bigint NOT NULL DEFAULT nextval('board_graveyard_id_seq'::regclass),
    CONSTRAINT boards_pkey PRIMARY KEY (id),
    CONSTRAINT deck_per_board UNIQUE (deck_id),
    CONSTRAINT graceyard_per_board UNIQUE (graveyard_id)
)

CREATE SEQUENCE IF NOT EXISTS public.unit_id_seq
    INCREMENT 1
    START 1
    MINVALUE 1
    MAXVALUE 32767;

CREATE TABLE IF NOT EXISTS public.units
(
    id smallint NOT NULL DEFAULT nextval('unit_id_seq'::regclass),
    colour text COLLATE pg_catalog."default" NOT NULL,
    CONSTRAINT units_pkey PRIMARY KEY (id)
)

CREATE SEQUENCE IF NOT EXISTS public.action_id_seq
    INCREMENT 1
    START 1
    MINVALUE 1
    MAXVALUE 32767;

CREATE TABLE IF NOT EXISTS public.actions
(
    id smallint NOT NULL DEFAULT nextval('action_id_seq'::regclass),
    action json NOT NULL,
    CONSTRAINT actions_pkey PRIMARY KEY (id)
)

CREATE SEQUENCE IF NOT EXISTS public.userhand_id_seq
    INCREMENT 1
    START 1
    MINVALUE 1
    MAXVALUE 9223372036854775807;

CREATE TABLE IF NOT EXISTS public.userhands
(
    id bigint NOT NULL DEFAULT nextval('userhand_id_seq'::regclass),
    user_id uuid NOT NULL,
    board_id uuid NOT NULL,
    CONSTRAINT userhands_pkey PRIMARY KEY (id),
    CONSTRAINT hand_per_game UNIQUE (user_id, board_id),
    CONSTRAINT fk_boards FOREIGN KEY (board_id)
        REFERENCES public.boards (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE,
    CONSTRAINT fk_users FOREIGN KEY (user_id)
        REFERENCES public.users (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE RESTRICT
)

CREATE TABLE IF NOT EXISTS public.userhand_items
(
    userhand_id bigint NOT NULL,
    card_id smallint NOT NULL,
    CONSTRAINT pk_userhand_item PRIMARY KEY (userhand_id, card_id)
)

CREATE SEQUENCE IF NOT EXISTS public.deck_item_order_seq
    INCREMENT 1
    START 1
    MINVALUE 1
    MAXVALUE 2147483647;

CREATE TABLE IF NOT EXISTS public.deck_items
(
    deck_id bigint NOT NULL,
    card_id smallint NOT NULL,
    order_idx integer NOT NULL DEFAULT nextval('deck_item_order_seq'::regclass),
    CONSTRAINT pk_deck_item PRIMARY KEY (deck_id, card_id),
    CONSTRAINT fk_actions FOREIGN KEY (card_id)
        REFERENCES public.actions (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE RESTRICT,
    CONSTRAINT fk_deck FOREIGN KEY (deck_id)
        REFERENCES public.boards (deck_id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
)

CREATE TABLE IF NOT EXISTS public.graveyard_items
(
    graveyard_id bigint NOT NULL,
    card_id smallint NOT NULL,
    CONSTRAINT pk_graveyard_item PRIMARY KEY (graveyard_id, card_id),
    CONSTRAINT fk_actions FOREIGN KEY (card_id)
        REFERENCES public.actions (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE RESTRICT,
    CONSTRAINT fk_graveyard FOREIGN KEY (graveyard_id)
        REFERENCES public.boards (graveyard_id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
)
