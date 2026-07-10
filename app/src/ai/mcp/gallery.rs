use std::collections::HashMap;

use chrono::DateTime;
use uuid::Uuid;
use cuteui::{Entity, ModelContext, SingletonEntity};

use crate::ai::mcp::templatable::{
    GalleryData, JsonTemplate, TemplatableMCPServer,
};

// DELETED: 云端功能 UpdateManager 相关导入已移除
// use crate::server::cloud_objects::{UpdateManager, UpdateManagerEvent};
use crate::server::datetime_ext::DateTimeExt;

#[derive(Clone, Debug)]
pub struct GalleryMCPServer {
    uuid: Uuid,
    title: String,
    description: String,
    #[allow(dead_code)]
    version: i32,
    #[allow(dead_code)]
    instructions_in_markdown: Option<String>,
    json_template: JsonTemplate,
}

impl GalleryMCPServer {
    pub fn new(
        uuid: Uuid,
        title: String,
        description: String,
        version: i32,
        instructions_in_markdown: Option<String>,
        json_template: JsonTemplate,
    ) -> Self {
        Self {
            uuid,
            title,
            description,
            version,
            instructions_in_markdown,
            json_template,
        }
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn description(&self) -> String {
        self.description.clone()
    }

    pub fn version(&self) -> i32 {
        self.version
    }

    pub fn json_template(&self) -> &JsonTemplate {
        &self.json_template
    }

    pub fn instructions_in_markdown(&self) -> Option<&String> {
        self.instructions_in_markdown.as_ref()
    }
}

impl TryFrom<GalleryMCPServer> for TemplatableMCPServer {
    type Error = String;

    fn try_from(gallery_server: GalleryMCPServer) -> Result<Self, Self::Error> {
        let GalleryMCPServer {
            uuid: gallery_uuid,
            title,
            description,
            version: gallery_version,
            instructions_in_markdown: _,
            json_template,
        } = gallery_server;

        Ok(TemplatableMCPServer {
            uuid: Uuid::new_v4(),
            name: title,
            description: Some(description),
            template: json_template,
            version: DateTime::now().timestamp(),
            gallery_data: Some(GalleryData {
                gallery_item_id: gallery_uuid,
                version: gallery_version,
            }),
        })
    }
}

impl From<TemplatableMCPServer> for GalleryMCPServer {
    fn from(templatable: TemplatableMCPServer) -> Self {
        let (uuid, version) = templatable
            .gallery_data
            .map(|g| (g.gallery_item_id, g.version))
            .unwrap_or((templatable.uuid, 0));

        GalleryMCPServer {
            uuid,
            title: templatable.name,
            description: templatable.description.unwrap_or_default(),
            version,
            instructions_in_markdown: None,
            json_template: templatable.template,
        }
    }
}

pub struct MCPGalleryManager {
    gallery_items: HashMap<Uuid, GalleryMCPServer>,
    templatable_mcp_servers: HashMap<Uuid, TemplatableMCPServer>,
}

impl MCPGalleryManager {
    pub fn new(ctx: &mut ModelContext<Self>) -> Self {
        let gallery_manager = Self {
            gallery_items: Default::default(),
            templatable_mcp_servers: Default::default(),
        };

        // COMMENTED: 云端功能 UpdateManager 订阅已禁用
        // // Subscribe to UpdateManager events to receive MCP gallery updates
        // let update_manager = UpdateManager::handle(ctx);
        // ctx.subscribe_to_model(&update_manager, |me, event, ctx| {
        //     if let UpdateManagerEvent::MCPGalleryUpdated { templates } = event {
        //         me.update_gallery_items(templates.clone(), ctx);
        //     }
        // });

        gallery_manager
    }

    pub fn get_gallery(&self) -> Vec<GalleryMCPServer> {
        self.gallery_items.values().cloned().collect()
    }

    pub fn get_gallery_item(&self, gallery_uuid: Uuid) -> Option<&GalleryMCPServer> {
        self.gallery_items.get(&gallery_uuid)
    }

    pub fn get_templatable_mcp_server(&self, gallery_uuid: Uuid) -> Option<&TemplatableMCPServer> {
        self.templatable_mcp_servers.get(&gallery_uuid)
    }

    /// Update gallery items from the server response
    pub fn update_gallery_items(
        &mut self,
        gallery_items: Vec<GalleryMCPServer>,
        ctx: &mut ModelContext<Self>,
    ) {
        let mut items = HashMap::new();
        let mut templatable_mcp_servers = HashMap::new();

        for gallery_item in gallery_items {
            let uuid = gallery_item.uuid();
            if let Ok(templatable) = TemplatableMCPServer::try_from(gallery_item.clone()) {
                items.insert(uuid, gallery_item);
                templatable_mcp_servers.insert(uuid, templatable);
            }
        }

        self.gallery_items = items;
        self.templatable_mcp_servers = templatable_mcp_servers;

        ctx.emit(MCPGalleryManagerEvent::ItemsRefreshed);
    }
}

pub enum MCPGalleryManagerEvent {
    ItemsRefreshed,
}

impl Entity for MCPGalleryManager {
    type Event = MCPGalleryManagerEvent;
}

impl SingletonEntity for MCPGalleryManager {}
