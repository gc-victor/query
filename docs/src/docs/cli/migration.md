# Migration

The migration command allows to manage the migrations of your Query Server, if you are admin.

Migration file:

- The migration file should be in the format of &lt;version&gt;*&lt;name&gt;*&lt;type&gt;.sql
- The version should be in the format of YYYYMMDD
- The name should be in the format of &lt;name&gt;_&lt;description&gt;
- The type should be up or down

Usage:  

```sh
query migration <DB_NAME> <PATH>
```
