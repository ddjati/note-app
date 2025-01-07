CREATE DATABASE IF NOT EXISTS rust_axum_sqlx;
USE rust_axum_sqlx;
CREATE TABLE IF NOT EXISTS notes (
    id CHAR(36) PRIMARY KEY NOT NULL,
    title VARCHAR(255) NOT NULL UNIQUE,
    content TEXT NOT NULL,
    is_published BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);
INSERT INTO notes (id, title, content, is_published) VALUES('f1cd96ca-0515-49de-be6d-3e238748668e', 'danangdjati note', 'here some reminder, mention @danangdjati', TRUE);
INSERT INTO notes (id, title, content, is_published) VALUES('ebc23e68-228f-474d-9f0d-d6f5714b5f92', 'yustiar note', 'here some reminder, mention @yustiar', TRUE);
INSERT INTO notes (id, title, content, is_published) VALUES('17865abf-331f-4197-8eb9-dcff465f5c36', 'dimas note', 'here some reminder, mention @dimas', TRUE);
INSERT INTO notes (id, title, content, is_published) VALUES('bcd93aea-53bc-4468-8a06-5c8080524c61', 'hafidh note', 'here some reminder, mention @hafidh', TRUE);
INSERT INTO notes (id, title, content, is_published) VALUES('4207cdf9-f4e5-4070-ac67-df41a406597a', 'habib note', 'here some reminder, mention @habib', TRUE);
INSERT INTO notes (id, title, content, is_published) VALUES('0bf1943f-15e7-4117-bf4d-f6790a241652', 'iqbal note', 'here some reminder, mention @iqbal', TRUE);
INSERT INTO notes (id, title, content, is_published) VALUES('8c4bb49e-e497-4a21-be8c-b5503b20ec54', 'cibong note', 'here some reminder, mention @cibong', TRUE);
INSERT INTO notes (id, title, content, is_published) VALUES('ffe83a90-1404-43a7-a0c5-f8c091e0d834', 'irsal note', 'here some reminder, mention @irsal', TRUE);
INSERT INTO notes (id, title, content, is_published) VALUES('1f544299-334d-4e71-a495-74e2c5365e88', 'avin note', 'here some reminder, mention @avin', TRUE);
INSERT INTO notes (id, title, content, is_published) VALUES('c4dd0646-1647-40f2-bd26-9f60f63f700c', 'Jo note', 'here some reminder, mention @Jo', TRUE);
INSERT INTO notes (id, title, content, is_published) VALUES('851073b2-1ebd-45dd-94be-b07e1f34b677', 'Betha note', 'here some reminder, mention @Betha', TRUE);
INSERT INTO notes (id, title, content, is_published) VALUES('41664a1d-34de-434f-bcdc-a5ee7d47eadb', 'Brian note', 'here some reminder, mention @Brian', TRUE);
INSERT INTO notes (id, title, content, is_published) VALUES('2f5e6455-1507-4ca3-b5d9-a53617ae2189', 'Bray note', 'here some reminder, mention @Bray', TRUE);
INSERT INTO notes (id, title, content, is_published) VALUES('3d861e1c-ad9c-435e-8fd1-c44def573e87', 'Ariel note', 'here some reminder, mention @Ariel', TRUE);
INSERT INTO notes (id, title, content, is_published) VALUES('1d81aa78-2835-4d51-bda0-8f3daa137bba', 'Arief note', 'here some reminder, mention @Arief', TRUE);
INSERT INTO notes (id, title, content, is_published) VALUES('5e946060-8422-48e0-9cd2-e64598a71f68', 'Bintar note', 'here some reminder, mention @Bintar', TRUE);
INSERT INTO notes (id, title, content, is_published) VALUES('c444531f-287f-47c6-ae0d-e94521fe836a', 'Brigitta note', 'here some reminder, mention @Brigitta', TRUE);