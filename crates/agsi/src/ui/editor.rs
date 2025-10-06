use agsi_core::{Document, Material, GroundModel, MaterialProperty};
use agsi_core::material::{MaterialType, PropertySource, PropertyValue};
use agsi_core::model::{ModelType, ModelDimension, ComponentType, ModelComponent};
use agsi_core::geometry::Geometry;
use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap, Clear},
    Frame, Terminal,
};
use std::path::PathBuf;

pub struct TuiEditor {
    document: Document,
    file_path: Option<PathBuf>,
    list_state: ListState,
    view_mode: ViewMode,
    creation_mode: Option<CreationMode>,
    input_buffer: String,
    status_message: String,
    should_quit: bool,
}

#[derive(Clone, Copy, PartialEq)]
enum ViewMode {
    DocumentInfo,
    Models,
    Materials,
    Components,
}

#[derive(Clone, PartialEq)]
enum CreationMode {
    CreateMaterial(MaterialCreationState),
    CreateModel(ModelCreationState),
    CreateComponent(ComponentCreationState),
    SelectMaterialType,
    SelectModelType,
    SelectModelDimension,
    SelectComponentType,
}

#[derive(Clone, PartialEq)]
struct MaterialCreationState {
    id: String,
    name: String,
    material_type: Option<MaterialType>,
    description: String,
    step: MaterialCreationStep,
}

#[derive(Clone, Copy, PartialEq)]
enum MaterialCreationStep {
    EnteringId,
    EnteringName,
    SelectingType,
    EnteringDescription,
}

#[derive(Clone, PartialEq)]
struct ModelCreationState {
    id: String,
    name: String,
    model_type: Option<ModelType>,
    dimension: Option<ModelDimension>,
    description: String,
    step: ModelCreationStep,
}

#[derive(Clone, Copy, PartialEq)]
enum ModelCreationStep {
    EnteringId,
    EnteringName,
    SelectingType,
    SelectingDimension,
    EnteringDescription,
}

#[derive(Clone, PartialEq)]
struct ComponentCreationState {
    id: String,
    name: String,
    component_type: Option<ComponentType>,
    material_id: String,
    model_index: usize,
    step: ComponentCreationStep,
}

#[derive(Clone, Copy, PartialEq)]
enum ComponentCreationStep {
    SelectingModel,
    EnteringId,
    EnteringName,
    SelectingType,
    EnteringMaterialId,
}

impl Default for MaterialCreationState {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            material_type: None,
            description: String::new(),
            step: MaterialCreationStep::EnteringId,
        }
    }
}

impl Default for ModelCreationState {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            model_type: None,
            dimension: None,
            description: String::new(),
            step: ModelCreationStep::EnteringId,
        }
    }
}

impl Default for ComponentCreationState {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            component_type: None,
            material_id: String::new(),
            model_index: 0,
            step: ComponentCreationStep::SelectingModel,
        }
    }
}

impl TuiEditor {
    pub fn new(document: Document, file_path: Option<PathBuf>) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        
        Self {
            document,
            file_path,
            list_state,
            view_mode: ViewMode::DocumentInfo,
            creation_mode: None,
            input_buffer: String::new(),
            status_message: "Press 'q' to quit, '?' for help, 'n' to create new".to_string(),
            should_quit: false,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Run app
        let res = self.run_app(&mut terminal);

        // Restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        if let Err(err) = res {
            eprintln!("Error: {:?}", err);
        }

        Ok(())
    }

    fn run_app<B: ratatui::backend::Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        loop {
            terminal.draw(|f| self.ui(f))?;

            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        self.handle_input(key.code);
                    }
                }
            }

            if self.should_quit {
                break;
            }
        }

        Ok(())
    }

    fn handle_input(&mut self, key: KeyCode) {
        // Handle creation mode inputs
        if let Some(mode) = self.creation_mode.clone() {
            self.handle_creation_input(key, mode);
            return;
        }

        // Normal mode inputs
        match key {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Char('d') => self.view_mode = ViewMode::DocumentInfo,
            KeyCode::Char('m') => self.view_mode = ViewMode::Models,
            KeyCode::Char('a') => self.view_mode = ViewMode::Materials,
            KeyCode::Char('c') => self.view_mode = ViewMode::Components,
            KeyCode::Char('n') => {
                // Start creation workflow
                self.start_creation_workflow();
            }
            KeyCode::Char('s') => {
                if let Some(ref path) = self.file_path {
                    match self.document.to_json_file(path) {
                        Ok(_) => self.status_message = format!("✅ Saved to {}", path.display()),
                        Err(e) => self.status_message = format!("❌ Error saving: {}", e),
                    }
                } else {
                    self.status_message = "❌ No file path specified".to_string();
                }
            }
            KeyCode::Down => {
                let len = self.get_list_len();
                if len > 0 {
                    let i = match self.list_state.selected() {
                        Some(i) => (i + 1) % len,
                        None => 0,
                    };
                    self.list_state.select(Some(i));
                }
            }
            KeyCode::Up => {
                let len = self.get_list_len();
                if len > 0 {
                    let i = match self.list_state.selected() {
                        Some(i) => if i == 0 { len - 1 } else { i - 1 },
                        None => 0,
                    };
                    self.list_state.select(Some(i));
                }
            }
            KeyCode::Char('?') => {
                self.status_message = "Keys: q=quit, d=doc, m=models, a=materials, c=components, n=new, s=save, ↑↓=navigate".to_string();
            }
            _ => {}
        }
    }

    fn start_creation_workflow(&mut self) {
        // Show menu based on current view
        match self.view_mode {
            ViewMode::Materials => {
                self.creation_mode = Some(CreationMode::CreateMaterial(MaterialCreationState::default()));
                self.status_message = "Creating Material: Enter ID (press Esc to cancel)".to_string();
            }
            ViewMode::Models => {
                self.creation_mode = Some(CreationMode::CreateModel(ModelCreationState::default()));
                self.status_message = "Creating Model: Enter ID (press Esc to cancel)".to_string();
            }
            ViewMode::Components => {
                if self.document.agsi_model.is_empty() {
                    self.status_message = "❌ Create a model first!".to_string();
                } else {
                    self.creation_mode = Some(CreationMode::CreateComponent(ComponentCreationState::default()));
                    self.status_message = "Creating Component: Select model (↑↓, Enter to confirm, Esc to cancel)".to_string();
                }
            }
            _ => {
                self.status_message = "Press: m=new model, a=new material, c=new component".to_string();
            }
        }
        self.input_buffer.clear();
    }

    fn handle_creation_input(&mut self, key: KeyCode, mode: CreationMode) {
        match key {
            KeyCode::Esc => {
                // Cancel creation
                self.creation_mode = None;
                self.input_buffer.clear();
                self.status_message = "Creation cancelled".to_string();
            }
            KeyCode::Enter => {
                self.handle_creation_enter(mode);
            }
            KeyCode::Backspace => {
                self.input_buffer.pop();
            }
            KeyCode::Char(c) => {
                self.input_buffer.push(c);
            }
            KeyCode::Down | KeyCode::Up => {
                // Handle selection navigation
                if let Some(CreationMode::CreateComponent(ref state)) = self.creation_mode {
                    if state.step == ComponentCreationStep::SelectingModel {
                        let len = self.document.agsi_model.len();
                        if len > 0 {
                            let current = self.list_state.selected().unwrap_or(0);
                            let next = if key == KeyCode::Down {
                                (current + 1) % len
                            } else {
                                if current == 0 { len - 1 } else { current - 1 }
                            };
                            self.list_state.select(Some(next));
                        }
                    }
                } else if let Some(CreationMode::SelectMaterialType) = self.creation_mode {
                    self.handle_type_selection_navigation(key);
                } else if let Some(CreationMode::SelectModelType) = self.creation_mode {
                    self.handle_type_selection_navigation(key);
                } else if let Some(CreationMode::SelectModelDimension) = self.creation_mode {
                    self.handle_type_selection_navigation(key);
                } else if let Some(CreationMode::SelectComponentType) = self.creation_mode {
                    self.handle_type_selection_navigation(key);
                }
            }
            _ => {}
        }
    }

    fn handle_type_selection_navigation(&mut self, key: KeyCode) {
        let len = 8; // Approximate number of types
        let current = self.list_state.selected().unwrap_or(0);
        let next = if key == KeyCode::Down {
            (current + 1) % len
        } else {
            if current == 0 { len - 1 } else { current - 1 }
        };
        self.list_state.select(Some(next));
    }

    fn handle_creation_enter(&mut self, mode: CreationMode) {
        match mode {
            CreationMode::CreateMaterial(mut state) => {
                match state.step {
                    MaterialCreationStep::EnteringId => {
                        state.id = self.input_buffer.clone();
                        self.input_buffer.clear();
                        state.step = MaterialCreationStep::EnteringName;
                        self.creation_mode = Some(CreationMode::CreateMaterial(state));
                        self.status_message = "Enter material name:".to_string();
                    }
                    MaterialCreationStep::EnteringName => {
                        state.name = self.input_buffer.clone();
                        self.input_buffer.clear();
                        state.step = MaterialCreationStep::SelectingType;
                        self.creation_mode = Some(CreationMode::SelectMaterialType);
                        self.list_state.select(Some(0));
                        self.status_message = "Select material type (↑↓, Enter):".to_string();
                    }
                    MaterialCreationStep::SelectingType => {
                        // Type already selected
                        state.step = MaterialCreationStep::EnteringDescription;
                        self.creation_mode = Some(CreationMode::CreateMaterial(state));
                        self.status_message = "Enter description (optional, press Enter to finish):".to_string();
                    }
                    MaterialCreationStep::EnteringDescription => {
                        state.description = self.input_buffer.clone();
                        self.finish_material_creation(state);
                    }
                }
            }
            CreationMode::SelectMaterialType => {
                let selected = self.list_state.selected().unwrap_or(0);
                let mat_type = match selected {
                    0 => MaterialType::Soil,
                    1 => MaterialType::Rock,
                    2 => MaterialType::Fill,
                    3 => MaterialType::MadeGround,
                    4 => MaterialType::Anthropogenic,
                    5 => MaterialType::Water,
                    6 => MaterialType::Void,
                    _ => MaterialType::Unknown,
                };
                
                if let Some(CreationMode::CreateMaterial(mut state)) = self.creation_mode.take() {
                    state.material_type = Some(mat_type);
                    state.step = MaterialCreationStep::EnteringDescription;
                    self.creation_mode = Some(CreationMode::CreateMaterial(state));
                    self.input_buffer.clear();
                    self.status_message = "Enter description (optional, press Enter to finish):".to_string();
                }
            }
            CreationMode::CreateModel(mut state) => {
                match state.step {
                    ModelCreationStep::EnteringId => {
                        state.id = self.input_buffer.clone();
                        self.input_buffer.clear();
                        state.step = ModelCreationStep::EnteringName;
                        self.creation_mode = Some(CreationMode::CreateModel(state));
                        self.status_message = "Enter model name:".to_string();
                    }
                    ModelCreationStep::EnteringName => {
                        state.name = self.input_buffer.clone();
                        self.input_buffer.clear();
                        state.step = ModelCreationStep::SelectingType;
                        self.creation_mode = Some(CreationMode::SelectModelType);
                        self.list_state.select(Some(0));
                        self.status_message = "Select model type (↑↓, Enter):".to_string();
                    }
                    ModelCreationStep::SelectingType => {
                        state.step = ModelCreationStep::SelectingDimension;
                        self.creation_mode = Some(CreationMode::SelectModelDimension);
                        self.list_state.select(Some(0));
                        self.status_message = "Select dimension (↑↓, Enter):".to_string();
                    }
                    ModelCreationStep::SelectingDimension => {
                        state.step = ModelCreationStep::EnteringDescription;
                        self.creation_mode = Some(CreationMode::CreateModel(state));
                        self.status_message = "Enter description (optional, press Enter to finish):".to_string();
                    }
                    ModelCreationStep::EnteringDescription => {
                        state.description = self.input_buffer.clone();
                        self.finish_model_creation(state);
                    }
                }
            }
            CreationMode::SelectModelType => {
                let selected = self.list_state.selected().unwrap_or(0);
                let model_type = match selected {
                    0 => ModelType::Stratigraphic,
                    1 => ModelType::Structural,
                    2 => ModelType::Hydrogeological,
                    3 => ModelType::Geotechnical,
                    4 => ModelType::Environmental,
                    _ => ModelType::Composite,
                };
                
                if let Some(CreationMode::CreateModel(mut state)) = self.creation_mode.take() {
                    state.model_type = Some(model_type);
                    state.step = ModelCreationStep::SelectingDimension;
                    self.creation_mode = Some(CreationMode::SelectModelDimension);
                    self.input_buffer.clear();
                    self.list_state.select(Some(0));
                    self.status_message = "Select dimension (↑↓, Enter):".to_string();
                }
            }
            CreationMode::SelectModelDimension => {
                let selected = self.list_state.selected().unwrap_or(0);
                let dimension = match selected {
                    0 => ModelDimension::OneD,
                    1 => ModelDimension::TwoD,
                    _ => ModelDimension::ThreeD,
                };
                
                if let Some(CreationMode::CreateModel(mut state)) = self.creation_mode.take() {
                    state.dimension = Some(dimension);
                    state.step = ModelCreationStep::EnteringDescription;
                    self.creation_mode = Some(CreationMode::CreateModel(state));
                    self.input_buffer.clear();
                    self.status_message = "Enter description (optional, press Enter to finish):".to_string();
                }
            }
            CreationMode::CreateComponent(mut state) => {
                match state.step {
                    ComponentCreationStep::SelectingModel => {
                        state.model_index = self.list_state.selected().unwrap_or(0);
                        state.step = ComponentCreationStep::EnteringId;
                        self.creation_mode = Some(CreationMode::CreateComponent(state));
                        self.status_message = "Enter component ID:".to_string();
                    }
                    ComponentCreationStep::EnteringId => {
                        state.id = self.input_buffer.clone();
                        self.input_buffer.clear();
                        state.step = ComponentCreationStep::EnteringName;
                        self.creation_mode = Some(CreationMode::CreateComponent(state));
                        self.status_message = "Enter component name:".to_string();
                    }
                    ComponentCreationStep::EnteringName => {
                        state.name = self.input_buffer.clone();
                        self.input_buffer.clear();
                        state.step = ComponentCreationStep::SelectingType;
                        self.creation_mode = Some(CreationMode::SelectComponentType);
                        self.list_state.select(Some(0));
                        self.status_message = "Select component type (↑↓, Enter):".to_string();
                    }
                    ComponentCreationStep::SelectingType => {
                        state.step = ComponentCreationStep::EnteringMaterialId;
                        self.creation_mode = Some(CreationMode::CreateComponent(state));
                        self.status_message = "Enter material ID:".to_string();
                    }
                    ComponentCreationStep::EnteringMaterialId => {
                        state.material_id = self.input_buffer.clone();
                        self.finish_component_creation(state);
                    }
                }
            }
            CreationMode::SelectComponentType => {
                let selected = self.list_state.selected().unwrap_or(0);
                let comp_type = match selected {
                    0 => ComponentType::Layer,
                    1 => ComponentType::Lens,
                    2 => ComponentType::Volume,
                    3 => ComponentType::Fault,
                    4 => ComponentType::Intrusion,
                    _ => ComponentType::Boundary,
                };
                
                if let Some(CreationMode::CreateComponent(mut state)) = self.creation_mode.take() {
                    state.component_type = Some(comp_type);
                    state.step = ComponentCreationStep::EnteringMaterialId;
                    self.creation_mode = Some(CreationMode::CreateComponent(state));
                    self.input_buffer.clear();
                    self.status_message = "Enter material ID:".to_string();
                }
            }
        }
    }

    fn finish_material_creation(&mut self, state: MaterialCreationState) {
        if let Some(mat_type) = state.material_type {
            let mut material = Material::new(state.id.clone(), state.name, mat_type);
            if !state.description.is_empty() {
                material.description = Some(state.description);
            }

            // Add to first model or create one if none exists
            if self.document.agsi_model.is_empty() {
                let mut model = GroundModel::new(
                    "MODEL001",
                    "Default Model",
                    ModelType::Geotechnical,
                    ModelDimension::TwoD,
                );
                model.add_material(material);
                self.document.add_model(model);
            } else {
                self.document.agsi_model[0].add_material(material);
            }

            self.creation_mode = None;
            self.input_buffer.clear();
            self.status_message = format!("✅ Material '{}' created!", state.id);
            self.view_mode = ViewMode::Materials;
        }
    }

    fn finish_model_creation(&mut self, state: ModelCreationState) {
        if let (Some(model_type), Some(dimension)) = (state.model_type, state.dimension) {
            let mut model = GroundModel::new(state.id.clone(), state.name, model_type, dimension);
            if !state.description.is_empty() {
                model.description = Some(state.description);
            }

            self.document.add_model(model);
            self.creation_mode = None;
            self.input_buffer.clear();
            self.status_message = format!("✅ Model '{}' created!", state.id);
            self.view_mode = ViewMode::Models;
        }
    }

    fn finish_component_creation(&mut self, state: ComponentCreationState) {
        if let Some(comp_type) = state.component_type {
            if state.model_index < self.document.agsi_model.len() {
                // Create simple point geometry as placeholder
                let geometry = Geometry::point(0.0, 0.0, 0.0);
                
                let component = ModelComponent::new(
                    state.id.clone(),
                    state.name,
                    comp_type,
                    &state.material_id,
                    geometry,
                );

                self.document.agsi_model[state.model_index].add_component(component);
                
                self.creation_mode = None;
                self.input_buffer.clear();
                self.status_message = format!("✅ Component '{}' created!", state.id);
                self.view_mode = ViewMode::Components;
            }
        }
    }

    fn get_list_len(&self) -> usize {
        match self.view_mode {
            ViewMode::Models => self.document.agsi_model.len(),
            ViewMode::Materials => {
                self.document.agsi_model.iter().map(|m| m.materials.len()).sum()
            }
            ViewMode::Components => {
                self.document.agsi_model.iter().map(|m| m.components.len()).sum()
            }
            _ => 0,
        }
    }

    fn ui(&mut self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(f.area());

        // Title
        self.render_title(f, chunks[0]);

        // Main content
        self.render_content(f, chunks[1]);

        // Status bar
        self.render_status(f, chunks[2]);

        // Render creation dialog if in creation mode
        if self.creation_mode.is_some() {
            self.render_creation_dialog(f);
        }
    }

    fn render_creation_dialog(&mut self, f: &mut Frame) {
        let area = f.area();
        
        // Create centered popup
        let popup_width = 60.min(area.width - 4);
        let popup_height = 20.min(area.height - 4);
        let popup_x = (area.width - popup_width) / 2;
        let popup_y = (area.height - popup_height) / 2;
        
        let popup_area = Rect::new(popup_x, popup_y, popup_width, popup_height);

        // Clear the area
        f.render_widget(Clear, popup_area);

        // Clone creation mode to avoid borrow issues
        let mode = self.creation_mode.clone();

        // Render based on creation mode
        match mode {
            Some(CreationMode::CreateMaterial(ref state)) => {
                self.render_material_creation_dialog(f, popup_area, state);
            }
            Some(CreationMode::SelectMaterialType) => {
                self.render_material_type_selector(f, popup_area);
            }
            Some(CreationMode::CreateModel(ref state)) => {
                self.render_model_creation_dialog(f, popup_area, state);
            }
            Some(CreationMode::SelectModelType) => {
                self.render_model_type_selector(f, popup_area);
            }
            Some(CreationMode::SelectModelDimension) => {
                self.render_dimension_selector(f, popup_area);
            }
            Some(CreationMode::CreateComponent(ref state)) => {
                self.render_component_creation_dialog(f, popup_area, state);
            }
            Some(CreationMode::SelectComponentType) => {
                self.render_component_type_selector(f, popup_area);
            }
            None => {}
        }
    }

    fn render_material_creation_dialog(&mut self, f: &mut Frame, area: Rect, state: &MaterialCreationState) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(0),
            ])
            .split(area);

        let block = Block::default()
            .title("Create Material")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Cyan));
        f.render_widget(block, area);

        // ID field
        let id_text = format!("ID: {}", if state.id.is_empty() { &self.input_buffer } else { &state.id });
        let id_widget = Paragraph::new(id_text)
            .style(if state.step == MaterialCreationStep::EnteringId {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default().fg(Color::Green)
            });
        f.render_widget(id_widget, chunks[0]);

        // Name field
        let name_text = format!("Name: {}", if state.name.is_empty() { &self.input_buffer } else { &state.name });
        let name_widget = Paragraph::new(name_text)
            .style(if state.step == MaterialCreationStep::EnteringName {
                Style::default().fg(Color::Yellow)
            } else if !state.name.is_empty() {
                Style::default().fg(Color::Green)
            } else {
                Style::default()
            });
        f.render_widget(name_widget, chunks[1]);

        // Type field
        let type_text = format!("Type: {:?}", state.material_type.unwrap_or(MaterialType::Unknown));
        let type_widget = Paragraph::new(type_text)
            .style(if state.material_type.is_some() {
                Style::default().fg(Color::Green)
            } else {
                Style::default()
            });
        f.render_widget(type_widget, chunks[2]);

        // Description field
        let desc_text = format!("Description: {}", if state.description.is_empty() { &self.input_buffer } else { &state.description });
        let desc_widget = Paragraph::new(desc_text)
            .style(if state.step == MaterialCreationStep::EnteringDescription {
                Style::default().fg(Color::Yellow)
            } else if !state.description.is_empty() {
                Style::default().fg(Color::Green)
            } else {
                Style::default()
            });
        f.render_widget(desc_widget, chunks[3]);
    }

    fn render_material_type_selector(&mut self, f: &mut Frame, area: Rect) {
        let items = vec![
            ListItem::new("Soil"),
            ListItem::new("Rock"),
            ListItem::new("Fill"),
            ListItem::new("Made Ground"),
            ListItem::new("Anthropogenic"),
            ListItem::new("Water"),
            ListItem::new("Void"),
            ListItem::new("Unknown"),
        ];

        let list = List::new(items)
            .block(Block::default()
                .title("Select Material Type")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Cyan)))
            .highlight_style(Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD))
            .highlight_symbol("▶ ");

        f.render_widget(Clear, area);
        f.render_stateful_widget(list, area, &mut self.list_state);
    }

    fn render_model_creation_dialog(&mut self, f: &mut Frame, area: Rect, state: &ModelCreationState) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(0),
            ])
            .split(area);

        let block = Block::default()
            .title("Create Model")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Cyan));
        f.render_widget(block, area);

        // ID
        let id_text = format!("ID: {}", if state.id.is_empty() { &self.input_buffer } else { &state.id });
        f.render_widget(Paragraph::new(id_text).style(
            if state.step == ModelCreationStep::EnteringId { Style::default().fg(Color::Yellow) } 
            else { Style::default().fg(Color::Green) }
        ), chunks[0]);

        // Name
        let name_text = format!("Name: {}", if state.name.is_empty() { &self.input_buffer } else { &state.name });
        f.render_widget(Paragraph::new(name_text).style(
            if state.step == ModelCreationStep::EnteringName { Style::default().fg(Color::Yellow) }
            else if !state.name.is_empty() { Style::default().fg(Color::Green) }
            else { Style::default() }
        ), chunks[1]);

        // Type
        let type_text = format!("Type: {:?}", state.model_type.unwrap_or(ModelType::Composite));
        f.render_widget(Paragraph::new(type_text).style(
            if state.model_type.is_some() { Style::default().fg(Color::Green) }
            else { Style::default() }
        ), chunks[2]);

        // Dimension
        let dim_text = format!("Dimension: {:?}", state.dimension.unwrap_or(ModelDimension::TwoD));
        f.render_widget(Paragraph::new(dim_text).style(
            if state.dimension.is_some() { Style::default().fg(Color::Green) }
            else { Style::default() }
        ), chunks[3]);

        // Description
        let desc_text = format!("Description: {}", if state.description.is_empty() { &self.input_buffer } else { &state.description });
        f.render_widget(Paragraph::new(desc_text).style(
            if state.step == ModelCreationStep::EnteringDescription { Style::default().fg(Color::Yellow) }
            else if !state.description.is_empty() { Style::default().fg(Color::Green) }
            else { Style::default() }
        ), chunks[4]);
    }

    fn render_model_type_selector(&mut self, f: &mut Frame, area: Rect) {
        let items = vec![
            ListItem::new("Stratigraphic"),
            ListItem::new("Structural"),
            ListItem::new("Hydrogeological"),
            ListItem::new("Geotechnical"),
            ListItem::new("Environmental"),
            ListItem::new("Composite"),
        ];

        let list = List::new(items)
            .block(Block::default()
                .title("Select Model Type")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Cyan)))
            .highlight_style(Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD))
            .highlight_symbol("▶ ");

        f.render_widget(Clear, area);
        f.render_stateful_widget(list, area, &mut self.list_state);
    }

    fn render_dimension_selector(&mut self, f: &mut Frame, area: Rect) {
        let items = vec![
            ListItem::new("1D - One Dimensional"),
            ListItem::new("2D - Two Dimensional"),
            ListItem::new("3D - Three Dimensional"),
        ];

        let list = List::new(items)
            .block(Block::default()
                .title("Select Dimension")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Cyan)))
            .highlight_style(Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD))
            .highlight_symbol("▶ ");

        f.render_widget(Clear, area);
        f.render_stateful_widget(list, area, &mut self.list_state);
    }

    fn render_component_creation_dialog(&mut self, f: &mut Frame, area: Rect, state: &ComponentCreationState) {
        if state.step == ComponentCreationStep::SelectingModel {
            // Show model selection
            let items: Vec<ListItem> = self.document.agsi_model.iter()
                .map(|m| ListItem::new(format!("{} ({})", m.name, m.id)))
                .collect();

            let list = List::new(items)
                .block(Block::default()
                    .title("Select Model for Component")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Cyan)))
                .highlight_style(Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD))
                .highlight_symbol("▶ ");

            f.render_widget(Clear, area);
            f.render_stateful_widget(list, area, &mut self.list_state);
        } else {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Min(0),
                ])
                .split(area);

            let block = Block::default()
                .title("Create Component")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Cyan));
            f.render_widget(block, area);

            // ID
            let id_text = format!("ID: {}", if state.id.is_empty() { &self.input_buffer } else { &state.id });
            f.render_widget(Paragraph::new(id_text).style(
                if state.step == ComponentCreationStep::EnteringId { Style::default().fg(Color::Yellow) }
                else { Style::default().fg(Color::Green) }
            ), chunks[0]);

            // Name
            let name_text = format!("Name: {}", if state.name.is_empty() { &self.input_buffer } else { &state.name });
            f.render_widget(Paragraph::new(name_text).style(
                if state.step == ComponentCreationStep::EnteringName { Style::default().fg(Color::Yellow) }
                else if !state.name.is_empty() { Style::default().fg(Color::Green) }
                else { Style::default() }
            ), chunks[1]);

            // Type
            let type_text = format!("Type: {:?}", state.component_type.unwrap_or(ComponentType::Layer));
            f.render_widget(Paragraph::new(type_text).style(
                if state.component_type.is_some() { Style::default().fg(Color::Green) }
                else { Style::default() }
            ), chunks[2]);

            // Material ID
            let mat_text = format!("Material ID: {}", if state.material_id.is_empty() { &self.input_buffer } else { &state.material_id });
            f.render_widget(Paragraph::new(mat_text).style(
                if state.step == ComponentCreationStep::EnteringMaterialId { Style::default().fg(Color::Yellow) }
                else if !state.material_id.is_empty() { Style::default().fg(Color::Green) }
                else { Style::default() }
            ), chunks[3]);
        }
    }

    fn render_component_type_selector(&mut self, f: &mut Frame, area: Rect) {
        let items = vec![
            ListItem::new("Layer"),
            ListItem::new("Lens"),
            ListItem::new("Volume"),
            ListItem::new("Fault"),
            ListItem::new("Intrusion"),
            ListItem::new("Boundary"),
        ];

        let list = List::new(items)
            .block(Block::default()
                .title("Select Component Type")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Cyan)))
            .highlight_style(Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD))
            .highlight_symbol("▶ ");

        f.render_widget(Clear, area);
        f.render_stateful_widget(list, area, &mut self.list_state);
    }

    fn render_title(&self, f: &mut Frame, area: Rect) {
        let title = Paragraph::new(vec![
            Line::from(vec![
                Span::styled("AGSi TUI Editor", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::raw(" - "),
                Span::raw(self.file_path.as_ref().map(|p| p.display().to_string()).unwrap_or_else(|| "New Document".to_string())),
            ]),
        ])
        .block(Block::default().borders(Borders::ALL));
        f.render_widget(title, area);
    }

    fn render_status(&self, f: &mut Frame, area: Rect) {
        let status = Paragraph::new(self.status_message.clone())
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Yellow));
        f.render_widget(status, area);
    }

    fn render_content(&mut self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(area);

        // Navigation menu
        self.render_menu(f, chunks[0]);

        // Content area
        match self.view_mode {
            ViewMode::DocumentInfo => self.render_document_info(f, chunks[1]),
            ViewMode::Models => self.render_models(f, chunks[1]),
            ViewMode::Materials => self.render_materials(f, chunks[1]),
            ViewMode::Components => self.render_components(f, chunks[1]),
        }
    }

    fn render_menu(&mut self, f: &mut Frame, area: Rect) {
        let menu_items = vec![
            ListItem::new(if self.view_mode == ViewMode::DocumentInfo { "▶ Document Info (d)" } else { "  Document Info (d)" }),
            ListItem::new(if self.view_mode == ViewMode::Models { "▶ Models (m)" } else { "  Models (m)" }),
            ListItem::new(if self.view_mode == ViewMode::Materials { "▶ Materials (a)" } else { "  Materials (a)" }),
            ListItem::new(if self.view_mode == ViewMode::Components { "▶ Components (c)" } else { "  Components (c)" }),
        ];

        let menu = List::new(menu_items)
            .block(Block::default().borders(Borders::ALL).title("Navigation"))
            .highlight_style(Style::default().bg(Color::DarkGray));

        f.render_widget(menu, area);
    }

    fn render_document_info(&self, f: &mut Frame, area: Rect) {
        let mut text = vec![
            Line::from(vec![Span::styled("Document Information", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))]),
            Line::from(""),
            Line::from(vec![
                Span::styled("File ID: ", Style::default().fg(Color::Green)),
                Span::raw(&self.document.ags_file.file_id),
            ]),
            Line::from(vec![
                Span::styled("Schema: ", Style::default().fg(Color::Green)),
                Span::raw(&self.document.ags_schema.version),
            ]),
        ];

        if let Some(ref author) = self.document.ags_file.file_author {
            text.push(Line::from(vec![
                Span::styled("Author: ", Style::default().fg(Color::Green)),
                Span::raw(author),
            ]));
        }

        text.push(Line::from(""));
        text.push(Line::from(vec![
            Span::styled("Models: ", Style::default().fg(Color::Yellow)),
                Span::raw(self.document.agsi_model.len().to_string()),
        ]));

        let total_materials: usize = self.document.agsi_model.iter().map(|m| m.materials.len()).sum();
        text.push(Line::from(vec![
            Span::styled("Materials: ", Style::default().fg(Color::Yellow)),
            Span::raw(total_materials.to_string()),
        ]));

        let total_components: usize = self.document.agsi_model.iter().map(|m| m.components.len()).sum();
        text.push(Line::from(vec![
            Span::styled("Components: ", Style::default().fg(Color::Yellow)),
            Span::raw(total_components.to_string()),
        ]));

        if let Some(ref project) = self.document.ags_project {
            text.push(Line::from(""));
            text.push(Line::from(vec![Span::styled("Project", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))]));
            text.push(Line::from(vec![
                Span::styled("  Name: ", Style::default().fg(Color::Green)),
                Span::raw(&project.name),
            ]));
            if let Some(ref client) = project.client {
                text.push(Line::from(vec![
                    Span::styled("  Client: ", Style::default().fg(Color::Green)),
                    Span::raw(client),
                ]));
            }
        }

        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title("Details"))
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, area);
    }

    fn render_models(&mut self, f: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self
            .document
            .agsi_model
            .iter()
            .map(|model| {
                ListItem::new(vec![
                    Line::from(vec![
                        Span::styled(&model.name, Style::default().fg(Color::Cyan)),
                        Span::raw(format!(" ({})", model.id)),
                    ]),
                    Line::from(format!("  Type: {:?}, Dimension: {:?}", model.model_type, model.dimension)),
                    Line::from(format!("  Materials: {}, Components: {}", model.materials.len(), model.components.len())),
                ])
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Models"))
            .highlight_style(Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD));

        f.render_stateful_widget(list, area, &mut self.list_state);
    }

    fn render_materials(&mut self, f: &mut Frame, area: Rect) {
        let mut items = Vec::new();
        for model in &self.document.agsi_model {
            for material in &model.materials {
                items.push(ListItem::new(vec![
                    Line::from(vec![
                        Span::styled(&material.name, Style::default().fg(Color::Yellow)),
                        Span::raw(format!(" ({})", material.id)),
                    ]),
                    Line::from(format!("  Type: {:?}", material.material_type)),
                    Line::from(format!("  Properties: {}", material.properties.len())),
                ]));
            }
        }

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Materials"))
            .highlight_style(Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD));

        f.render_stateful_widget(list, area, &mut self.list_state);
    }

    fn render_components(&mut self, f: &mut Frame, area: Rect) {
        let mut items = Vec::new();
        for model in &self.document.agsi_model {
            for component in &model.components {
                items.push(ListItem::new(vec![
                    Line::from(vec![
                        Span::styled(&component.name, Style::default().fg(Color::Magenta)),
                        Span::raw(format!(" ({})", component.id)),
                    ]),
                    Line::from(format!("  Type: {:?}", component.component_type)),
                    Line::from(format!("  Material: {}", component.material_id)),
                ]));
            }
        }

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Components"))
            .highlight_style(Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD));

        f.render_stateful_widget(list, area, &mut self.list_state);
    }
}

pub async fn launch_editor(file: Option<PathBuf>) -> Result<()> {
    let doc = if let Some(ref path) = file {
        Document::from_json_file(path)?
    } else {
        Document::default()
    };

    let mut editor = TuiEditor::new(doc, file);
    editor.run()?;

    Ok(())
}
