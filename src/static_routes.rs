use cc_server_kit::prelude::*;
use cc_server_kit::utils::prelude::*;

const LOCAL_FRONTEND_DISTRIBUTABLE: &str = "dist/";
const CONTAINER_FRONTEND_DISTRIBUTABLE: &str = "/usr/local/frontend-dist/";

pub async fn get_filepath_from_dist(filename: impl Into<String>) -> MResult<String> {
  let filename = filename.into();
  tracing::debug!("Пытаемся получить доступ к файлу {}", filename);

  let filepath = format!("{}{}", CONTAINER_FRONTEND_DISTRIBUTABLE, &filename);
  if tokio::fs::try_exists(&filepath).await? {
    return Ok(filepath);
  }
  let filepath = format!("{}{}", LOCAL_FRONTEND_DISTRIBUTABLE, &filename);
  if tokio::fs::try_exists(&filepath).await? {
    return Ok(filepath);
  }

  Err(
    ErrorResponse::from(format!(r#"Не удалось открыть файл "{}""#, filename))
      .with_404_pub()
      .build(),
  )
}

pub async fn get_from_dist(filename: impl Into<String>) -> MResult<File> {
  let filename = filename.into();
  let filepath = get_filepath_from_dist(&filename).await?;
  file_upload!(filepath, filename)
}

#[handler]
#[tracing::instrument(skip_all, fields(http.uri = req.uri().path(), http.method = req.method().as_str()))]
pub async fn frontend(req: &Request) -> MResult<Html> {
  let filepath = get_filepath_from_dist("index.html").await?;
  let site = tokio::fs::read_to_string(&filepath).await?;
  html!(site)
}

#[handler]
#[tracing::instrument(skip_all, fields(http.uri = req.uri().path(), http.method = req.method().as_str()))]
pub async fn get_uikit_app_internals(req: &Request) -> MResult<AnyOf> {
  let rest_path = req
    .param::<String>("rest_path")
    .ok_or("Не удалось получить остальной путь.")?;
  match get_from_dist(rest_path).await {
    Ok(file) => Ok(AnyOf::File(file)),
    Err(_) => match frontend::frontend(req).await {
      Ok(html) => Ok(AnyOf::Html(html)),
      Err(e) => Err(ErrorResponse::from(e).with_404_pub().build()),
    },
  }
}

enum AnyOf {
  Html(Html),
  File(File),
}

#[salvo::async_trait]
impl salvo::Writer for AnyOf {
  async fn write(self, req: &mut Request, depot: &mut Depot, res: &mut salvo::Response) {
    match self {
      AnyOf::Html(html) => html.write(req, depot, res).await,
      AnyOf::File(file) => file.write(req, depot, res).await,
    }
  }
}

pub fn frontend_router() -> Router {
  Router::new()
    .get(frontend)
    .push(Router::with_path("<**rest_path>").get(get_uikit_app_internals))
}
