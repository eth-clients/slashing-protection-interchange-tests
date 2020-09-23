Draft of slashing protection interchange tests for the format described here:

https://hackmd.io/@sproul/Bk0Y0qdGD

## How to run

Each test directory contains an interchange file and some extra data about how to test it.

For example:

```json
{
  "name": "single_validator_genesis_attestation",
  "should_succeed": true,
  "genesis_validators_root": "0x0000000000000000000000000000000000000000000000000000000000000000",
  "interchange": {
    "metadata": {
      "interchange_format": "complete",
      "interchange_format_version": "4",
      "genesis_validators_root": "0x0000000000000000000000000000000000000000000000000000000000000000"
    },
    "data": [
      {
        "pubkey": "0xa99a76ed7796f7be22d5b7e85deeb7c5677e88e511e0b337618f8c4eb61349b4bf2d153f649f7b53359fe8b94a38e44c",
        "signed_blocks": [],
        "signed_attestations": [
          {
            "source_epoch": "0",
            "target_epoch": "0"
          }
        ]
      }
    ]
  },
  "blocks": [],
  "attestations": [
    {
      "pubkey": "0xa99a76ed7796f7be22d5b7e85deeb7c5677e88e511e0b337618f8c4eb61349b4bf2d153f649f7b53359fe8b94a38e44c",
      "source_epoch": 0,
      "target_epoch": 0,
      "should_succeed": false
    }
  ]
}
```

To run a test, import the data from the `interchange` field into a newly initialized (empty)
slashing protection database. Then, determine the test outcome according to the meanings of
each of the fields, which are as follows:

* `name: string`: the name of the test-case, informational.
* `should_succeed: bool`: whether the `interchange` given is valid and should
  be imported successfully.
* `genesis_validators_root: Root`: the genesis validators root to use when
  creating the empty slashing protection database, or to compare the import
  against.
* `interchange: Interchange`: slashing protection interchange data as described
  by the spec.
* `blocks: [object]`: a list of block signings to be attempted **after**
  importing the `interchange`, detailed below.
* `attestations: [object]`: a list of attestation signings to be attempted **after**
  importing the `interchange`, detailed below.

Each block in `blocks` is structured as:

```json
{
  "pubkey": "0xa99a76ed7796f7be22d5b7e85deeb7c5677e88e511e0b337618f8c4eb61349b4bf2d153f649f7b53359fe8b94a38e44c",
  "slot": 1,
  "should_succeed": false
}
```

Your test-runner should attempt to sign a block at the given slot from the
given `pubkey` (use a random `signing_root`/block body if necessary). The
`should_succeed` field describes whether this signing should be accepted (true)
or rejected (false). Clients using a slashing protection mechanism that admits
false positives may consider a rejection as success even if `should_succeed` is
true.

Each attestation in `attestations` is structured as:

```json
{
  "pubkey": "0xa99a76ed7796f7be22d5b7e85deeb7c5677e88e511e0b337618f8c4eb61349b4bf2d153f649f7b53359fe8b94a38e44c",
  "source_epoch": 11,
  "target_epoch": 12,
  "should_succeed": true
}
```

Similarly to above, your test-runner should attempt to sign an attestation with these parameters
using the given `pubkey`, and suceed based on the value of `should_succeed`. Again, false positives
may be treated as successes.

Note that the top-level `genesis_validators_root` is not necessarily the same
as the GVR contained in the interchange, to allow us to test the case where
they are mismatched.

## Downloading the tests

For the time being you can hit this URL to get a .tar.gz of this repo:

```
https://github.com/eth2-clients/slashing-protection-interchange-tests/tarball/<COMMIT_HASH>
```

Or you can use a git submodule.
