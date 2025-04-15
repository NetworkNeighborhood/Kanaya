use std::{fs, path::PathBuf};
use fluent::{FluentArgs, FluentBundle, FluentResource};
use fluent_fallback::{
    generator::{BundleGenerator, FluentBundleResult},
    types::ResourceId,
    Localization,
};
use rustc_hash::FxHashSet;
use unic_langid::{langid, LanguageIdentifier};

pub struct FluentResourceManager {
    resource_path_scheme: PathBuf
}

impl FluentResourceManager {
    pub fn new(path_schema: String) -> Self {
        FluentResourceManager { resource_path_scheme: PathBuf::from(&path_schema) }
    }
}

pub struct BundleIterator {
    resource_path_scheme: String,
    locales: <Vec<LanguageIdentifier> as IntoIterator>::IntoIter,
    resource_ids: FxHashSet<ResourceId>
}

impl Iterator for BundleIterator {
    type Item = FluentBundleResult<FluentResource>;
    
    fn next(&mut self) -> Option<Self::Item> {
        let locale = self.locales.next()?;
        let res_path_scheme = self
            .resource_path_scheme
            .as_str()
            .replace("{locale}", &locale.to_string());
        
        let mut bundle = FluentBundle::new(vec![locale]);
        
        let mut errors = vec![];
        
        for resource_id in &self.resource_ids {
            let resource_path = res_path_scheme.as_str().replace("{res_id}", &resource_id.value);
            let source = fs::read_to_string(resource_path).unwrap();
            let res = match FluentResource::try_new(source) {
                Ok(res) => res,
                Err((res, err)) => {
                    errors.extend(err.into_iter().map(Into::into));
                    res
                }
            };
            bundle.add_resource(res).unwrap();
        }
        
        if errors.is_empty() {
            Some(Ok(bundle))
        }
        else {
            Some(Err((bundle, errors)))
        }
    }
}

impl futures::Stream for BundleIterator {
    type Item = FluentBundleResult<FluentResource>;
    
    fn poll_next(self: std::pin::Pin<&mut Self>, _cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        unimplemented!()
    }
}

impl BundleGenerator for FluentResourceManager {
    type Resource = FluentResource;
    type LocalesIter = std::vec::IntoIter<LanguageIdentifier>;
    type Iter = BundleIterator;
    type Stream = BundleIterator;
    
    fn bundles_iter(&self, locales: Self::LocalesIter, res_ids: FxHashSet<ResourceId>) -> Self::Iter {
        BundleIterator {
            resource_path_scheme: self.resource_path_scheme.to_string_lossy().to_string(),
            locales,
            resource_ids: res_ids,
        }
    }
    
    fn bundles_stream(
        &self,
        _locales: Self::LocalesIter,
        _res_ids: FxHashSet<ResourceId>,
    ) -> Self::Stream {
        std::unimplemented!();
    }
}