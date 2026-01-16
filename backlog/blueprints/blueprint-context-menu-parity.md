# Blueprint: Context Menu Feature Parity

**Bug ID:** FAUGUS-RS-001  
**Priority:** P1 (High)  
**Estimated Effort:** 1.5-2 hours  
**Status:** [x] Completed

---

## Problem Statement

The context menu in the Rust version displays only **3 items** instead of **10 items** as in the Python version. This breaks feature parity and limits user functionality.

| Version | Items Count | Actions Available |
|---------|-------------|-------------------|
| Python (GTK3) | 10 | Full functionality |
| Rust (Iced) | 3 | Only file/prefix/logs operations |

---

## Phase 1: Extend ContextMenuMessage Enum [x]

**Goal**: Add missing message variants to support all context menu actions.

**Resource Context**:
- 📄 `src/gui/context_menu.rs` (lines 5-10)
- 📄 `src/main.rs` (lines 42-95) — existing `Message` enum for reference

**Steps**:
1. [x] Add new variants to `ContextMenuMessage` enum:
   - `Play`
   - `Edit`
   - `Delete`
   - `Duplicate`
   - `ToggleHidden`
2. [x] Run `cargo-check` to verify enum compiles (will show unused warnings — expected).

> [!NOTE]
> The existing variants `OpenLocation`, `OpenPrefix`, `ShowLogs` must remain unchanged.

---

## Phase 2: Extend ContextMenu Struct [x]

**Goal**: Store game metadata needed for display (title, playtime, hidden state).

**Resource Context**:
- 📄 `src/gui/context_menu.rs` (lines 12-24)
- 📄 `src/config/game_config.rs` — `Game` struct with `title`, `playtime`, `hidden` fields

**Steps**:
1. [x] Add fields to `ContextMenu` struct:
   - `game_title: String`
   - `playtime_formatted: String`
   - `is_hidden: bool`
2. [x] Update `ContextMenu::new()` constructor to accept these parameters.
3. [x] Run `cargo-check` to verify struct changes.

> [!IMPORTANT]
> Use `Game::format_playtime()` method (already exists in `game_config.rs:172`) to format playtime string.

---

## Phase 3: Update ContextMenu::view() Method [x]

**Goal**: Render all 10 menu items matching Python version structure.

**Resource Context**:
- 📄 `src/gui/context_menu.rs` (lines 26-51)
- 📄 `faugus_launcher.py` (lines 236-276) — Python menu structure reference
- 📚 **Docs**: `iced::widget::button`, `iced::widget::text`, `iced::widget::horizontal_rule`

**Menu Structure** (matches Python version order):
```
┌─────────────────────────┐
│ Game Title (disabled)   │  <- text, not button
│ Playtime (disabled)     │  <- text, not button
├─────────────────────────┤  <- horizontal_rule or spacing
│ Play                    │  -> ContextMenuMessage::Play
│ Edit                    │  -> ContextMenuMessage::Edit
│ Delete                  │  -> ContextMenuMessage::Delete
│ Duplicate               │  -> ContextMenuMessage::Duplicate
│ Hide / Show             │  -> ContextMenuMessage::ToggleHidden
├─────────────────────────┤
│ Open game location      │  -> ContextMenuMessage::OpenLocation
│ Open prefix location    │  -> ContextMenuMessage::OpenPrefix
│ Show logs               │  -> ContextMenuMessage::ShowLogs
└─────────────────────────┘
```

**Steps**:
1. [x] Add header section with `game_title` and `playtime_formatted` as disabled text.
2. [x] Add visual separator (use `horizontal_rule()` or increased spacing).
3. [x] Add action buttons: Play, Edit, Delete, Duplicate.
4. [x] Add Hide/Show button with **dynamic label** based on `is_hidden`:
   - If `is_hidden == true` → label = `"Show"`
   - If `is_hidden == false` → label = `"Hide"`
5. [x] Keep existing buttons: Open game location, Open prefix location, Show logs.
6. [x] Run `cargo-check` to verify view compiles.

> [!NOTE]
> All button labels must use `i18n.t("key")` for localization support.

---

## Phase 4: Update ContextMenu Creation Site [x]

**Goal**: Pass game data when creating the context menu.

**Resource Context**:
- 📄 `src/main.rs` (lines 264-267) — `Message::GameRightClicked` handler

**Steps**:
1. [x] In `Message::GameRightClicked` handler, retrieve game by index.
2. [x] Extract `title`, call `format_playtime()`, get `hidden` state.
3. [x] Pass extracted data to `ContextMenu::new()`.
4. [x] Handle edge case: if game not found at index, do not show menu.
5. [x] Run `cargo-check` to verify integration.

---

## Phase 5: Route New Messages to Existing Handlers [x]

**Goal**: Connect context menu actions to already-implemented handlers in main.rs.

**Resource Context**:
- 📄 `src/main.rs` (lines 269-295) — `Message::ContextMenu` handler
- 📄 `src/main.rs` — existing handlers:
  - `Message::PlayClicked` (line 47)
  - `Message::ShowEditGameDialog` (line 68)
  - `Message::DeleteClicked` (line 228)
  - `Message::DuplicateClicked` (line 591)
  - `Message::HideShowClicked` (line 564)

**Message Routing Table**:
| ContextMenuMessage | Action | Target Message |
|--------------------|--------|----------------|
| `Play` | Close menu, launch game | Direct launch via `main_window` |
| `Edit` | Close menu, open edit dialog | `ShowEditGameDialog(game_index)` |
| `Delete` | Close menu, trigger delete flow | Set selected + `DeleteClicked` |
| `Duplicate` | Close menu, duplicate game | Set selected + `DuplicateClicked` |
| `ToggleHidden` | Close menu, toggle visibility | Set selected + `HideShowClicked` |

**Steps**:
1. [x] Add match arms for new `ContextMenuMessage` variants.
2. [x] For `Play`: Close dialog, call game launch directly (similar to double-click).
3. [x] For `Edit`: Close dialog, return `Task::done(Message::ShowEditGameDialog(game_index))`.
4. [x] For `Delete`, `Duplicate`, `ToggleHidden`:
   - Set `main_window.select_game(game_index)` to ensure correct game is targeted.
   - Close dialog.
   - Return appropriate `Task::done(Message::*)`.
5. [x] Run `cargo-check` to verify all match arms are exhaustive.

> [!IMPORTANT]
> Existing handlers (`DeleteClicked`, `DuplicateClicked`, `HideShowClicked`) rely on `selected_game_index()`. Ensure the game is selected before dispatching these messages.

---

## Phase 6: Verify Localization Keys [x]

**Goal**: Ensure all new UI strings have translation keys.

**Resource Context**:
- 📄 `src/locale/` — i18n system

**Required Keys**:
- `"Play"`
- `"Edit"`
- `"Delete"`
- `"Duplicate"`
- `"Hide"`
- `"Show"`

**Steps**:
1. [x] Check if keys exist in locale files.
2. [x] Add missing keys if necessary.
3. [x] Verify fallback behavior for missing translations.

---

## Phase 7: Final QA [x]

**Goal**: Verify implementation compiles and passes lints.

**Steps**:
1. [x] Run `cargo-check --all-targets` — must pass with no errors.
2. [x] Run `cargo-clippy` — fix any warnings.
3. [x] Run `cargo-fmt` — ensure code style.
4. [x] Manual test: right-click on game, verify all 10 items appear.
5. [x] Manual test: verify each action works correctly.

---

## Invariants & Safety Bounds

1. **No Breaking Changes**: Existing 3 menu items must continue to work.
2. **Index Safety**: Always validate `game_index` before accessing games vector.
3. **State Consistency**: Close dialog before dispatching actions to avoid stale state.
4. **Localization**: All user-facing strings must use `i18n.t()`.

---

## Files Modified Summary

| File | Changes |
|------|---------|
| `src/gui/context_menu.rs` | Extend enum, struct, and view() |
| `src/main.rs` | Update GameRightClicked and ContextMenu handlers |
| `src/locale/*` | Add missing translation keys (if needed) |

---

## Dependencies

All required infrastructure already exists:
- `Game::format_playtime()` — implemented
- `Game::hidden` field — implemented
- `Message::DeleteClicked`, `DuplicateClicked`, `HideShowClicked` handlers — implemented
- `Message::ShowEditGameDialog` — implemented
- Game launch logic — implemented (via double-click handler)
