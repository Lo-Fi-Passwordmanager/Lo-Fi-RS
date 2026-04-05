# Lo-Fi RS

&rarr; [deutsch](https://github.com/Lo-Fi-Passwordmanager/Lo-Fi-RS/blob/main/README.md)

A local-first password manager built around the [Automerge](https://automerge.org/) library.

A library to access the password databases of
the [Lo-Fi Password Manager](https://github.com/Lo-Fi-Passwordmanager/Lo-Fi-Passwordmanager).

Uses Automerge as its foundation, [autosurgeon](https://github.com/automerge/autosurgeon) to serialize the Automerge
documents as Rust data structures, and [samod](https://github.com/alexjg/samod) as an interface to the automerge repo,
for example, to connect to sync servers (
&rarr; [automerge-repo-sync-server](https://github.com/automerge/automerge-repo-sync-server)).

## Development

To begin development, the repository must first be cloned. Furthermore, to use the
synchronization functionality, an Automerge sync server is required (
&rarr; [automerge-repo-sync-server](https://github.com/automerge/automerge-repo-sync-server)). A public Automerge server
is provided; this is set as the default in the app, but it can be slow or experience outages.

Rustc >= 1.94.1 is also required to run the project ( &rarr; [rust](https://rust-lang.org/tools/install/)).

The relevant part of the development takes place in `lofi/src/`, where the actual code that interacts with Automerge
resides. The `main.rs` file is largely irrelevant; it is currently primarily used for testing the library during
development, but a fully functional CLI is planned for this area.

### Data Structure

The schema for the JSON structure persisted by Automerge is described by the following schema:

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "title": "AutomergeDoc",
  "type": "object",
  "required": [
    "salt",
    "validation",
    "items"
  ],
  "properties": {
    "salt": {
      "type": "string",
      "minLength": 64,
      "maxLength": 64
    },
    "validation": {
      "type": "string",
      "minLength": 185,
      "maxLength": 185
    },
    "items": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/AutomergeItem"
      }
    }
  },
  "$defs": {
    "AutomergeItem": {
      "type": "object",
      "required": [
        "type",
        "name",
        "createdAt",
        "editedAt",
        "parentId"
      ],
      "properties": {
        "name": {
          "type": "string"
        },
        "createdAt": {
          "type": "number",
          "description": "Unix timestamp"
        },
        "editedAt": {
          "type": "number",
          "description": "Unix timestamp"
        },
        "parentId": {
          "type": "string"
        }
      },
      "oneOf": [
        {
          "$ref": "#/$defs/AutomergeFolder"
        },
        {
          "$ref": "#/$defs/AutomergeEntry"
        }
      ]
    },
    "AutomergeFolder": {
      "properties": {
        "type": {
          "const": "folder"
        }
      }
    },
    "AutomergeEntry": {
      "required": [
        "username",
        "password",
        "url",
        "note"
      ],
      "properties": {
        "type": {
          "const": "entry"
        },
        "username": {
          "type": "string"
        },
        "password": {
          "type": "string"
        },
        "url": {
          "type": "string"
        },
        "note": {
          "type": "string"
        }
      }
    }
  }
}
```