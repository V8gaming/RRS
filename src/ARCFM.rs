use tui::buffer::Buffer;
use tui::layout::{Constraint, Layout, Rect, Direction};
use tui::style::Style;
use tui::widgets::{Block, Borders, Cell, Widget, TableState, StatefulWidget};
use tui::text::Text;
use unicode_width::UnicodeWidthStr;

#[derive(Clone,Copy)]
pub struct AbsorberRodControlAndFuelMonitoring {
    reactor_fuel_rod_matrix: [[FuelRod; 5];5],
    pull_rods: bool,
    insert_rods: bool,
    hold_rods: bool,
    // slow, medium, fast
    speed_setpoint: u8,
    reactivity: f32,
    centre_core_only: bool,
}

#[derive(Clone, Copy)]
pub struct FuelRod {
    absorber_rod_position: f32,
    fuel_temperature: f32,
    thermal_power_output: f32,
    pull_rod: bool,
    insert_rod: bool,
}


impl Default for AbsorberRodControlAndFuelMonitoring {
    fn default() -> Self {
        Self {
            reactor_fuel_rod_matrix: [[FuelRod::default(); 5]; 5],
            pull_rods: false,
            insert_rods: false,
            hold_rods: false,
            speed_setpoint: 0,
            reactivity: 0.0,
            centre_core_only: false,
        }
    }
}

impl Default for FuelRod {
    fn default() -> Self {
        Self {
            absorber_rod_position: 0.0,
            fuel_temperature: 0.0,
            thermal_power_output: 0.0,
            pull_rod: false,
            insert_rod: false,
        }
    }
}

pub fn fuel_rod_table_row(starting_number: i32, width: i32) -> LocalRow<'static> {
    let mut cell_vec = Vec::new();
    //println!("starting number: {}", starting_number+1);
    for i in starting_number+1..starting_number+width+1 {
        //println!("cell number: {}", i);
        let text = Text::from(i.to_string());
        let cell = LocalCell::from(text).style(tui::style::Style::default().fg(tui::style::Color::Yellow));
        cell_vec.push(cell);
    }
    LocalRow::new(cell_vec)
    
}

pub fn fuel_rod_table(width: i32, height: i32) -> TableVec<'static>{
    let mut row_vec = Vec::new();
    let mut constraint_vec = Vec::new();
    for i in 1..height {
        //println!("height: {}", i);
        //println!("{} * {}: {}", i, height, i*height);
        row_vec.push(fuel_rod_table_row(i*height, width));
        constraint_vec.push(Constraint::Percentage((100/height) as u16));
    }
    TableVec::new(row_vec).block(Block::default().title("Fuel Rods").borders(Borders::ALL)).widths(constraint_vec)
    .style(
        tui::style::Style::default().fg(tui::style::Color::Yellow),
    )
}   
pub struct TableVec<'a> {
    /// A block to wrap the widget in
    block: Option<Block<'a>>,
    /// Base style for the widget
    style: Style,
    /// Width constraints for each column
    widths: Vec<Constraint>,
    /// Space between each column
    column_spacing: u16,
    /// Style used to render the selected row
    highlight_style: Style,
    /// Symbol in front of the selected rom
    highlight_symbol: Option<&'a str>,
    /// Optional header
    header: Option<LocalRow<'a>>,
    /// Data to display in each row
    rows: Vec<LocalRow<'a>>,
}
impl <'a>TableVec<'a> {
    pub fn new<T>(rows: T) -> Self
    where
    T: IntoIterator<Item = LocalRow<'a>>,
    {        
        Self {
        block: None,
        style: Style::default(),
        widths: vec![],
        column_spacing: 1,
        highlight_style: Style::default(),
        highlight_symbol: None,
        header: None,
        rows: rows.into_iter().collect(),
    }
}

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    pub fn header(mut self, header: LocalRow<'a>) -> Self {
        self.header = Some(header);
        self
    }
    pub fn widths(mut self, widths: Vec<Constraint>) -> Self {
        let between_0_and_100 = |&w| match w {
            Constraint::Percentage(p) => p <= 100,
            _ => true,
        };
        assert!(
            widths.iter().all(between_0_and_100),
            "Percentages should be between 0 and 100 inclusively."
        );
        self.widths = widths;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn highlight_symbol(mut self, highlight_symbol: &'a str) -> Self {
        self.highlight_symbol = Some(highlight_symbol);
        self
    }

    pub fn highlight_style(mut self, highlight_style: Style) -> Self {
        self.highlight_style = highlight_style;
        self
    }

    pub fn column_spacing(mut self, spacing: u16) -> Self {
        self.column_spacing = spacing;
        self
    }

    fn get_columns_widths(&self, max_width: u16, has_selection: bool) -> Vec<u16> {
        let mut constraints = Vec::with_capacity(self.widths.len() * 2 + 1);
        if has_selection {
            let highlight_symbol_width =
                self.highlight_symbol.map(|s| s.width() as u16).unwrap_or(0);
            constraints.push(Constraint::Length(highlight_symbol_width));
        }
        for constraint in self.widths {
            constraints.push(constraint);
            constraints.push(Constraint::Length(self.column_spacing));
        }
        if !self.widths.is_empty() {
            constraints.pop();
        }
        let mut chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            //.expand_to_fill(false)
            .split(Rect {
                x: 0,
                y: 0,
                width: max_width,
                height: 1,
            });
        if has_selection {
            chunks.remove(0);
        }
        chunks.iter().step_by(2).map(|c| c.width).collect()
    }

    fn get_row_bounds(
        &self,
        selected: Option<usize>,
        offset: usize,
        max_height: u16,
    ) -> (usize, usize) {
        let offset = offset.min(self.rows.len().saturating_sub(1));
        let mut start = offset;
        let mut end = offset;
        let mut height = 0;
        for item in self.rows.iter().skip(offset) {
            if height + item.height > max_height {
                break;
            }
            height += item.total_height();
            end += 1;
        }

        let selected = selected.unwrap_or(0).min(self.rows.len() - 1);
        while selected >= end {
            height = height.saturating_add(self.rows[end].total_height());
            end += 1;
            while height > max_height {
                height = height.saturating_sub(self.rows[start].total_height());
                start += 1;
            }
        }
        while selected < start {
            start -= 1;
            height = height.saturating_add(self.rows[start].total_height());
            while height > max_height {
                end -= 1;
                height = height.saturating_sub(self.rows[end].total_height());
            }
        }
        (start, end)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LocalRow<'a> {
    cells: Vec<LocalCell<'a>>,
    height: u16,
    style: Style,
    bottom_margin: u16,
}

impl<'a> LocalRow<'a> {
    /// Creates a new [`Row`] from an iterator where items can be converted to a [`Cell`].
    pub fn new<T>(cells: T) -> Self
    where
        T: IntoIterator,
        T::Item: Into<LocalCell<'a>>,
    {
        Self {
            height: 1,
            cells: cells.into_iter().map(|c| c.into()).collect(),
            style: Style::default(),
            bottom_margin: 0,
        }
    }

    /// Set the fixed height of the [`Row`]. Any [`Cell`] whose content has more lines than this
    /// height will see its content truncated.
    pub fn height(mut self, height: u16) -> Self {
        self.height = height;
        self
    }

    /// Set the [`Style`] of the entire row. This [`Style`] can be overriden by the [`Style`] of a
    /// any individual [`Cell`] or event by their [`Text`] content.
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Set the bottom margin. By default, the bottom margin is `0`.
    pub fn bottom_margin(mut self, margin: u16) -> Self {
        self.bottom_margin = margin;
        self
    }

    /// Returns the total height of the row.
    fn total_height(&self) -> u16 {
        self.height.saturating_add(self.bottom_margin)
    }
}
impl<'a> Widget for TableVec<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = LocalTableState::default();
        StatefulWidget::render(self, area, buf, &mut state);
    }
}
impl<'a> StatefulWidget for TableVec<'a> {
    type State = LocalTableState;

    fn render(mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        if area.area() == 0 {
            return;
        }
        buf.set_style(area, self.style);
        let table_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        let has_selection = state.selected.is_some();
        let columns_widths = self.get_columns_widths(table_area.width, has_selection);
        let highlight_symbol = self.highlight_symbol.unwrap_or("");
        let blank_symbol = " ".repeat(highlight_symbol.width());
        let mut current_height = 0;
        let mut rows_height = table_area.height;

        // Draw header
        if let Some(ref header) = self.header {
            let max_header_height = table_area.height.min(header.total_height());
            buf.set_style(
                Rect {
                    x: table_area.left(),
                    y: table_area.top(),
                    width: table_area.width,
                    height: table_area.height.min(header.height),
                },
                header.style,
            );
            let mut col = table_area.left();
            if has_selection {
                col += (highlight_symbol.width() as u16).min(table_area.width);
            }
            for (width, cell) in columns_widths.iter().zip(header.cells.iter()) {
                render_cell(
                    buf,
                    cell,
                    Rect {
                        x: col,
                        y: table_area.top(),
                        width: *width,
                        height: max_header_height,
                    },
                );
                col += *width + self.column_spacing;
            }
            current_height += max_header_height;
            rows_height = rows_height.saturating_sub(max_header_height);
        }

        // Draw rows
        if self.rows.is_empty() {
            return;
        }
        let (start, end) = self.get_row_bounds(state.selected, state.offset, rows_height);
        state.offset = start;
        for (i, table_row) in self
            .rows
            .iter_mut()
            .enumerate()
            .skip(state.offset)
            .take(end - start)
        {
            let (row, col) = (table_area.top() + current_height, table_area.left());
            current_height += table_row.total_height();
            let table_row_area = Rect {
                x: col,
                y: row,
                width: table_area.width,
                height: table_row.height,
            };
            buf.set_style(table_row_area, table_row.style);
            let is_selected = state.selected.map(|s| s == i).unwrap_or(false);
            let table_row_start_col = if has_selection {
                let symbol = if is_selected {
                    highlight_symbol
                } else {
                    &blank_symbol
                };
                let (col, _) =
                    buf.set_stringn(col, row, symbol, table_area.width as usize, table_row.style);
                col
            } else {
                col
            };
            let mut col = table_row_start_col;
            for (width, cell) in columns_widths.iter().zip(table_row.cells.iter()) {
                render_cell(
                    buf,
                    cell,
                    Rect {
                        x: col,
                        y: row,
                        width: *width,
                        height: table_row.height,
                    },
                );
                col += *width + self.column_spacing;
            }
            if is_selected {
                buf.set_style(table_row_area, self.highlight_style);
            }
        }
    }
}


#[derive(Debug, Clone, Default)]
pub struct LocalTableState {
    offset: usize,
    selected: Option<usize>,
}

impl LocalTableState {
    pub fn selected(&self) -> Option<usize> {
        self.selected
    }

    pub fn select(&mut self, index: Option<usize>) {
        self.selected = index;
        if index.is_none() {
            self.offset = 0;
        }
    }
}

fn render_cell(buf: &mut Buffer, cell: &LocalCell, area: Rect) {
    buf.set_style(area, cell.style);
    for (i, spans) in cell.content.lines.iter().enumerate() {
        if i as u16 >= area.height {
            break;
        }
        buf.set_spans(area.x, area.y + i as u16, spans, area.width);
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LocalCell<'a> {
    content: Text<'a>,
    style: Style,
}

impl<'a> LocalCell<'a> {
    /// Set the `Style` of this cell.
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl<'a, T> From<T> for LocalCell<'a>
where
    T: Into<Text<'a>>,
{
    fn from(content: T) -> LocalCell<'a> {
        LocalCell {
            content: content.into(),
            style: Style::default(),
        }
    }
}