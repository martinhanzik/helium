pub struct Position {
    line: u32,
    column: u32
}

pub enum Cursor {
    Simple(Position),
    Range(Position, Position),
    Block(Position, Position)
}

pub trait StrLike {
    type Bytes: Iterator;
    type Chars: Iterator;
    type Lines: Iterator;

    /// Return number of bytes in this object
    fn len(&self) -> usize;
    /// Return number of unicode characters in this object
    fn char_count(&self) -> usize;
    /// Return number of lines in this object
    fn line_count(&self) -> usize;

    fn bytes(&self) -> Bytes;
    fn chars(&self) -> Chars;
    fn lines(&self) -> Lines;

    // TODO: strchr or some such. Bytes or chars based?
    // TODO: Storing styles
}

pub trait Snapshot: StrLike {
    fn cursors(&self) -> &[Cursor];

    /// Return a snapshot on which this one was based or None if it's not available
    fn previous(&self) -> Option<Self>;

    /// Return all snapshots based on this one
    fn next(&self) -> &[Self];
}

pub trait Buffer {
    type SnapshotType: Snapshot;

    fn current_snapshot(&self) -> Snapshot;
}
