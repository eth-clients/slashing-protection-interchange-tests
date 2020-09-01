Rough draft of slashing protection interchange tests for the format described here:

https://hackmd.io/@sproul/Bk0Y0qdGD

How to run:

* For each JSON file in `minimal/import/valid`:
    * Parse the file and import it into a _new_ slashing protection database.
    * The test succeeds iff no error occurs during parsing or import.
    * Optional: re-export the database and check that it matches the input file.
* For each JSON file in `minimal/import/invalid`:
    * Attempt to parse the file and, if successful, attempt to import it into a new slashing
      protection database.
    * The test succeeds iff an error occurs during parsing or import.
