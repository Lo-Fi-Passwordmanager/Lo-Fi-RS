# Lo-Fi RS

&rarr; [english](https://github.com/Lo-Fi-Passwordmanager/Lo-Fi-RS/blob/main/README.en.md)

Ein Local-First Passwortmanager aufgebaut um die Bibliothek [Automerge](https://automerge.org/).

Eine Bibliothek um auf Passwortdatenbanken
des [Lo-Fi Passwortmanagers](https://github.com/Lo-Fi-Passwordmanager/Lo-Fi-Passwordmanager) zuzugreifen.

Nutzt im Hintergrund Automerge als Basis, [autosurgeon](https://github.com/automerge/autosurgeon) um die Automerge
Dokument als Rust Datenstruktur zu Serialisieren
und [samod](https://github.com/alexjg/samod) als Schnittstelle mit automerge-repo, um sich bspw.
mit Syncservern (
&rarr; [automerge-repo-sync-server](https://github.com/automerge/automerge-repo-sync-server)) zu verbinden.

## Entwicklung

Um mit der Entwicklung zu starten, muss zunächst das Repository geklont werden. Weiterhin wird zum verwenden der
Synchronisationsfunktionalität ein Automerge-Syncserver benötigt (
&rarr; [automerge-repo-sync-server](https://github.com/automerge/automerge-repo-sync-server)). Es wird ein öffentlicher
Server von Automerge bereitgestellt, dieser ist in der App als default hinterlegt, kann jedoch langsam sein oder
ausfälle haben.

Es wird weiterhin rustc >= 1.94.1 benötigt, um das Projekt auszuführen (
&rarr; [rust](https://rust-lang.org/tools/install/)).

Der relevate Teil der Entwicklung findet in `lofi/src/` statt, hier liegt der eigentliche Code, der mit automerge
interargiert. Die `main.rs` ist größtenteils nicht relevant, sie ist im moment noch hauptsächlich zum Testen der
Bibliothek während der Entwicklung zuständig, hier soll allerdings mal ein vollwertiges cli entstehen.

### Datenstruktur

Das Schema für die JSON-Struktur, die von Automerge persistiert wird, wird von Folgendem Schema beschrieben:

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