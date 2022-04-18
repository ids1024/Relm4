use proc_macro2::TokenStream as TokenStream2;
use syn::{Path, Type, Visibility};

use crate::widgets::{PropertyType, ReturnedWidget, Widget};

#[derive(Default)]
pub(super) struct TokenStreams {
    /// The tokens for the struct fields -> name: Type,
    pub struct_fields: TokenStream2,
    /// The tokens initializing the widgets.
    pub init_widgets: TokenStream2,
    /// The tokens initializing the properties.
    pub assign_properties: TokenStream2,
    /// The tokens connecting widgets.
    pub connect_widgets: TokenStream2,
    /// The tokens for the returned struct fields -> name,
    pub return_fields: TokenStream2,
    /// The view tokens (watch! macro)
    pub view: TokenStream2,
    /// The view tokens (track! macro)
    pub track: TokenStream2,
    /// The tokens for connecting events.
    pub connect: TokenStream2,
    /// The tokens for connecting events to components.
    pub connect_components: TokenStream2,
}

impl Widget {
    pub(super) fn generate_factory_prototype_tokens_recursively(
        &self,
        streams: &mut TokenStreams,
        vis: &Option<Visibility>,
        model_type: &Type,
        relm4_path: &Path,
    ) {
        self.struct_fields_stream(&mut streams.struct_fields, vis);
        self.init_widgets_stream(&mut streams.init_widgets);
        self.return_stream(&mut streams.return_fields);

        for prop in &self.properties.properties {
            prop.connect_widgets_stream(&mut streams.connect_widgets, &self.name, relm4_path);

            if let PropertyType::Widget(widget) = &prop.ty {
                widget.generate_factory_prototype_tokens_recursively(
                    streams, vis, model_type, relm4_path,
                );
                if let Some(returned_widget) = &widget.returned_widget {
                    returned_widget.generate_factory_prototype_tokens_recursively(
                        streams, vis, model_type, relm4_path,
                    );
                }
            } else {
                prop.property_init_stream(&mut streams.assign_properties, &self.name, relm4_path);

                prop.view_stream(&mut streams.view, &self.name, relm4_path, true);
                prop.track_stream(&mut streams.track, &self.name, model_type, true, relm4_path);

                prop.connect_stream(&mut streams.connect, &self.name, relm4_path);
                prop.connect_component_stream(
                    &mut streams.connect_components,
                    &self.name,
                    relm4_path,
                );

                // prop.connect_parent_stream(&mut streams.parent, &self.name);
            }
        }
    }
}

impl ReturnedWidget {
    pub(super) fn generate_factory_prototype_tokens_recursively(
        &self,
        streams: &mut TokenStreams,
        vis: &Option<Visibility>,
        model_type: &Type,
        relm4_path: &Path,
    ) {
        self.struct_fields_stream(&mut streams.struct_fields, vis);
        self.return_stream(&mut streams.return_fields);

        for prop in &self.properties.properties {
            prop.connect_widgets_stream(&mut streams.connect_widgets, &self.name, relm4_path);

            if let PropertyType::Widget(widget) = &prop.ty {
                widget.generate_factory_prototype_tokens_recursively(
                    streams, vis, model_type, relm4_path,
                );
            } else {
                prop.property_init_stream(&mut streams.assign_properties, &self.name, relm4_path);
                prop.connect_stream(&mut streams.connect, &self.name, relm4_path);

                prop.view_stream(&mut streams.view, &self.name, relm4_path, true);
                prop.track_stream(&mut streams.track, &self.name, model_type, true, relm4_path);

                prop.connect_component_stream(
                    &mut streams.connect_components,
                    &self.name,
                    relm4_path,
                );
            }
        }
    }
}
