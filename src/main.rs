use ncurses::*;

enum WindowEvent
{
    Move 
    {
        y: i32,
        x: i32
    },
    Resize
    {
        lines: i32,
        cols: i32,
    },
    Create
    {
        lines: i32,
        cols: i32,
        y: i32,
        x: i32,
    },
    Quit,
}

struct Point
{
    y: i32,
    x: i32,
}

struct DIM
{
    lines: i32,
    cols: i32,
}

struct WinState
{
    win: WINDOW,
    size: DIM,
    pos: Point,
    sub_win: Vec<WinState>,
    quit: bool,
}

impl WinState
{
    fn create_subwin(&mut self, line_in: i32, col_in: i32, y_in: i32, x_in: i32)
    {
        self.sub_win.push(
            WinState
            {
                win: newwin(line_in, col_in, y_in, x_in),
                size: DIM {lines: line_in, cols: col_in},
                pos: Point {y: 0, x: 0},
                sub_win: Vec::<WinState>::new(),
                quit: false,
            }
            );
    }

    fn resize(&mut self, lines:i32, cols:i32)
    {
        wresize(self.win, lines, cols);
    }

    fn set_pos(&mut self, y_in:i32, x_in: i32)
    {
        self.pos = Point{y: y_in, x: x_in};
        wmove(self.win, self.pos.y, self.pos.x);
    }

    fn quit(&mut self)
    {
        self.quit = true;
    }

    fn process(&mut self, event: WindowEvent)
    {
        match event
        {
            WindowEvent::Move {y, x} =>
            {
                self.set_pos(y, x);
            }
            WindowEvent::Resize {lines, cols} =>
            {
                self.resize(lines, cols);
            }
            WindowEvent::Create {lines, cols, y, x} =>
            {
                self.create_subwin(lines, cols, y, x);
            }
            WindowEvent::Quit =>
            {
                self.quit();
            }
        }
    }
}

fn resized(win: WINDOW, cols:&mut i32, lines:&mut i32)
{
    if is_term_resized(*cols, *lines)
    {
        getmaxyx(win, lines, cols);
        wresize(win, *lines, *cols);
    }
}

fn get_scr() -> WINDOW
{
    let stdscr:WINDOW = initscr();
    keypad(stdscr, true);
    noecho();
    raw();
    return stdscr;
}

fn main()
{
    let tmp = get_scr();
    let mut tmplines:i32 = 0 ;
    let mut tmpcols:i32 = 0 ;
    getmaxyx(tmp, &mut tmplines, &mut tmpcols);
    let mut stdscr = WinState
    {
        win: tmp,
        size: DIM {lines: tmplines, cols: tmpcols},
        pos: Point {y: 0, x: 0},
        sub_win: Vec::<WinState>::new(),
        quit: false,
    };
    loop
    {
        stdscr.process(WindowEvent::Move {y: stdscr.pos.y+1, x: stdscr.pos.x+1});
        waddstr(stdscr.win, "test");
        let input = wgetch(stdscr.win) as u8 as char;
        if input == 'q' || input == 'Q'
        {
            break;
        }
    }
    echo();
    noraw();
    endwin();
}

