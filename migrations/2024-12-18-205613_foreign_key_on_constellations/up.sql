ALTER TABLE "star"
    ADD CONSTRAINT fk_constellation_id_star FOREIGN KEY (constellation_id)
        REFERENCES constellation (id);
