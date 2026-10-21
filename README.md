# rebolt

Rust implementation of [`rebulk`](https://github.com/Toilal/rebulk), a rule engine for hierarchical string matching and conflict resolution.

> **Early development.** The API will change.

## What it does

- Regex and literal rules, with names and priorities.
- Hierarchical matches: a regex's named capture groups become child matches. `S02E05` yields both the whole token and its `season`/`episode` parts.
- Overlap detection between matches, with a pluggable resolver that decides who survives.
- `cdylib` + `rlib` output.

## Usage

```rust
use rebolt::rules::Text;
use rebolt::{Rebolt, Re};

let engine = Rebolt::new()
    .rule(Re::new("episode_info", r"S(?P<season>\d{2})E(?P<episode>\d{2})").priority(10))
    .rule(Re::new("year", r"(?P<year>\d{4})").priority(1))
    .rule(Text::new("language", "VF").priority(1));

let matches = engine.matches("Doctor.Who.S02E05.2005.VF");

matches.named("episode_info");     // "S02E05"
matches.named_children("season");  // "02"
matches.named_children("episode"); // "05"
matches.named("year");             // "2005"
matches.named("language");         // "VF"
```

`named()` returns root matches, `named_children()` returns matches produced as part of a
parent regex. Both return empty vectors when nothing matched.

### Custom resolver

Every conflicting pair is handed to the resolver as a `ConflictCursor` exposing `left()` and
`right()`; it answers `KeepLeft`, `KeepRight` or `Ignore`. The default `SimpleResolver` sorts
by priority first, then by span width.

```rust
use rebolt::resolver::Resolver;
use rebolt::types::{ConflictContext, ConflictDecision};

struct PreferLeft;

impl Resolver for PreferLeft {
    fn resolve(&self, context: &mut ConflictContext) {
        context.for_each_conflict(|_cursor| ConflictDecision::KeepLeft);
    }
}

let engine = Rebolt::new().resolver(PreferLeft);
```

## API

| Type | Role |
|------|------|
| `Rebolt` | Engine. `.rule()`, `.resolver()`, `.matches()` |
| `Re` | Regex rule; named groups become child matches |
| `Text` | Literal substring rule |
| `Match` | `id`, `name`, `priority`, `content`, `range`, `parent_id`, `child_ids` |
| `Matches` | Result set: `matches`, `by_name`, `order`, `named()`, `named_children()` |
| `Resolver` | Trait for conflict strategies |
| `SimpleResolver` | Default: priority, then width |
| `ConflictContext` / `ConflictCursor` | What a resolver is given |

`Match` exposes `is_root()`, `is_internal()` and `is_leaf()`.

## License

MIT
