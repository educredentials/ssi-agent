CREATE TABLE IF NOT EXISTS events
(
    aggregate_type text                         NOT NULL,
    aggregate_id   text                         NOT NULL,
    sequence       bigint CHECK (sequence >= 0) NOT NULL,
    event_type     text                         NOT NULL,
    event_version  text                         NOT NULL,
    payload        json                         NOT NULL,
    metadata       json                         NOT NULL,
    PRIMARY KEY (aggregate_type, aggregate_id, sequence)
);

CREATE TABLE IF NOT EXISTS connection
(
    view_id           text                        NOT NULL,
    version           bigint CHECK (version >= 0) NOT NULL,
    payload           json                        NOT NULL,
    PRIMARY KEY (view_id)
);

CREATE TABLE IF NOT EXISTS all_connections
(
    view_id           text                        NOT NULL,
    version           bigint CHECK (version >= 0) NOT NULL,
    payload           json                        NOT NULL,
    PRIMARY KEY (view_id)
);

CREATE TABLE IF NOT EXISTS document
(
    view_id           text                        NOT NULL,
    version           bigint CHECK (version >= 0) NOT NULL,
    payload           json                        NOT NULL,
    PRIMARY KEY (view_id)
);

CREATE TABLE IF NOT EXISTS all_documents
(
    view_id           text                        NOT NULL,
    version           bigint CHECK (version >= 0) NOT NULL,
    payload           json                        NOT NULL,
    PRIMARY KEY (view_id)
);

CREATE TABLE IF NOT EXISTS profile
(
    view_id           text                        NOT NULL,
    version           bigint CHECK (version >= 0) NOT NULL,
    payload           json                        NOT NULL,
    PRIMARY KEY (view_id)
);

CREATE TABLE IF NOT EXISTS service
(
    view_id           text                        NOT NULL,
    version           bigint CHECK (version >= 0) NOT NULL,
    payload           json                        NOT NULL,
    PRIMARY KEY (view_id)
);

CREATE TABLE IF NOT EXISTS all_services
(
    view_id           text                        NOT NULL,
    version           bigint CHECK (version >= 0) NOT NULL,
    payload           json                        NOT NULL,
    PRIMARY KEY (view_id)
);

CREATE TABLE IF NOT EXISTS offer
(
    view_id           text                        NOT NULL,
    version           bigint CHECK (version >= 0) NOT NULL,
    payload           json                        NOT NULL,
    PRIMARY KEY (view_id)
);

CREATE TABLE IF NOT EXISTS all_offers
(
    view_id           text                        NOT NULL,
    version           bigint CHECK (version >= 0) NOT NULL,
    payload           json                        NOT NULL,
    PRIMARY KEY (view_id)
);

CREATE TABLE IF NOT EXISTS pre_authorized_code
(
    view_id           text                        NOT NULL,
    version           bigint CHECK (version >= 0) NOT NULL,
    payload           json                        NOT NULL,
    PRIMARY KEY (view_id)
);

CREATE TABLE IF NOT EXISTS access_token
(
    view_id           text                        NOT NULL,
    version           bigint CHECK (version >= 0) NOT NULL,
    payload           json                        NOT NULL,
    PRIMARY KEY (view_id)
);

CREATE TABLE IF NOT EXISTS credential
(
    view_id           text                        NOT NULL,
    version           bigint CHECK (version >= 0) NOT NULL,
    payload           json                        NOT NULL,
    PRIMARY KEY (view_id)
);

CREATE TABLE IF NOT EXISTS all_credentials
(
    view_id           text                        NOT NULL,
    version           bigint CHECK (version >= 0) NOT NULL,
    payload           json                        NOT NULL,
    PRIMARY KEY (view_id)
);

CREATE TABLE IF NOT EXISTS server_config
(
    view_id           text                        NOT NULL,
    version           bigint CHECK (version >= 0) NOT NULL,
    payload           json                        NOT NULL,
    PRIMARY KEY (view_id)
);

CREATE TABLE IF NOT EXISTS received_offer
(
    view_id           text                        NOT NULL,
    version           bigint CHECK (version >= 0) NOT NULL,
    payload           json                        NOT NULL,
    PRIMARY KEY (view_id)
);

CREATE TABLE IF NOT EXISTS all_received_offers
(
    view_id           text                        NOT NULL,
    version           bigint CHECK (version >= 0) NOT NULL,
    payload           json                        NOT NULL,
    PRIMARY KEY (view_id)
);

CREATE TABLE IF NOT EXISTS holder_credential
(
    view_id           text                        NOT NULL,
    version           bigint CHECK (version >= 0) NOT NULL,
    payload           json                        NOT NULL,
    PRIMARY KEY (view_id)
);

CREATE TABLE IF NOT EXISTS presentation
(
    view_id           text                        NOT NULL,
    version           bigint CHECK (version >= 0) NOT NULL,
    payload           json                        NOT NULL,
    PRIMARY KEY (view_id)
);

CREATE TABLE IF NOT EXISTS all_presentations
(
    view_id           text                        NOT NULL,
    version           bigint CHECK (version >= 0) NOT NULL,
    payload           json                        NOT NULL,
    PRIMARY KEY (view_id)
);

CREATE TABLE IF NOT EXISTS all_holder_credentials
(
    view_id           text                        NOT NULL,
    version           bigint CHECK (version >= 0) NOT NULL,
    payload           json                        NOT NULL,
    PRIMARY KEY (view_id)
);

CREATE TABLE IF NOT EXISTS authorization_request
(
    view_id           text                        NOT NULL,
    version           bigint CHECK (version >= 0) NOT NULL,
    payload           json                        NOT NULL,
    PRIMARY KEY (view_id)
);

CREATE TABLE IF NOT EXISTS all_authorization_requests
(
    view_id           text                        NOT NULL,
    version           bigint CHECK (version >= 0) NOT NULL,
    payload           json                        NOT NULL,
    PRIMARY KEY (view_id)
);