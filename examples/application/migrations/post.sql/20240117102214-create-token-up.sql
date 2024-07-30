INSERT
OR IGNORE INTO _config_token (name, token, expiration_date, write)
VALUES
    (
        'post',
        token (
            '{
                    "sub": "' || (
                SELECT
                    uuid ()
            ) || '",
                    "exp": ' || strftime ('%s', datetime ('now')) || ',
                    "iat": ' || strftime ('%s', datetime ('now')) || ',
                    "iss": "token"
                }'
        ),
        strftime ('%s', datetime ('now')),
        1
    );
