#[derive(Responder)]
#[response(status = 418, content_type = "json")]
pub struct RawTeapotJson<QuoteOutput> {
    pub output: QuoteOutput,
}
