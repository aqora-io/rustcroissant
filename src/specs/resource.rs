use crate::specs::{
    common::{Id, Iri, NonEmptyString, Ref, UrlOrRelativePath},
    serde::one_or_many,
};
use schematic::Config;

config_enum!(
    /// Croissant datasets contain data. Resources describe how that data is
    /// organized. Croissant defines two types of resources:
    ///
    /// - `FileObject` for individual files that are part of a dataset.
    /// - `FileSet` for homogeneous sets of files that are part of the
    ///   dataset (e.g., a directory of images).
    ///
    /// While [schema.org/Dataset](http://schema.org/Dataset) defines a
    /// `distribution` property, it's insufficient to adequately represent
    /// the contents of a dataset, as each distribution corresponds to a
    /// single downloadable form of the dataset. In practice, datasets
    /// often use `distribution` to represent separate files that are part
    /// of the dataset, but that is technically not a correct use of the
    /// property, and is still insufficient to describe datasets with a
    /// more complex layout, which is often the case of ML datasets.
    ///
    /// In Croissant, the `distribution` property contains one or more
    /// `FileObject` or `FileSet` instead of schema.org's `DataDownload`.
    #[derive(Config)]
    #[serde(tag = "@type")]
    pub enum Resource {
        /// `FileObject` is the Croissant class used to represent individual
        /// files that are part of a dataset.
        ///
        /// `FileObject` is a general purpose class that inherits from
        /// [Schema.org](http://schema.org) `DataDownload`, and can be used to
        /// represent instances of more specific types of content like
        /// `DigitalDocument` and `MediaObject`.
        ///
        /// Most of the important properties needed to describe a `FileObject`
        /// are defined in the classes it inherits from.
        #[serde(rename = "cr:FileObject")]
        FileObject(FileObject),

        /// In many datasets, data comes in the form of collections of
        /// homogeneous files, such as images, videos, or text files, where
        /// each file needs to be treated as an individual item, e.g., as a
        /// training example. `FileSet` is a class that describes such
        /// collections of files.
        ///
        /// A `FileSet` is a set of files located in a container, which can be
        /// an archive `FileObject` or a "manifest" file. A `FileSet` may also
        /// specify inclusion / exclusion filters: these are file patterns that
        /// give the user flexibility to define which files should be part of
        /// the `FileSet`. For example, include patterns may refer to all images
        /// under one or more directories, while exclude patterns may be used
        /// to exclude specific images.
        ///
        /// `FileSet` also extends `sc:DataDownload`, and defines the following
        /// additional properties.
        #[serde(rename = "cr:FileSet")]
        FileSet(FileSet),
    }
);

impl Resource {
    pub fn id(&self) -> &Id {
        match self {
            Resource::FileObject(o) => &o.id,
            Resource::FileSet(s) => &s.id,
        }
    }
}

config_struct!(
    #[derive(Config)]
    pub struct FileObject {
        #[serde(rename = "@id")]
        pub id: Id,

        /// The name of the file. As much as possible, the name should reflect
        /// the name of the file as downloaded, including the file extension,
        /// e.g. "images.zip".
        pub name: Option<NonEmptyString>,

        /// Actual bytes of the media object, for example the image file
        /// or video file.
        pub content_url: UrlOrRelativePath,

        /// File size in (mega/kilo/...)bytes. Defaults to bytes if a unit is not
        /// specified.
        ///
        /// Valid formats: "1024", "1024 B", "1024 KB", "1024 MB", etc.
        #[setting(validate = schematic::validate::regex("^[0-9]+( [KMGT]?B)?$"))]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub content_size: Option<String>,

        /// The formats of the file, given as a mime type. Unregistered or niche
        /// encoding and file formats can be indicated instead via the most
        /// appropriate URL, e.g. a defining Web page or a Wikipedia/Wikidata
        /// entry.
        pub encoding_format: Iri,

        /// URL (or local name) of a FileObject with the same content, but in a
        /// different format.
        #[serde(
            skip_serializing_if = "Vec::is_empty",
            default,
            deserialize_with = "one_or_many"
        )]
        pub same_as: Vec<Iri>,

        /// Checksum for the file contents.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sha256: Option<NonEmptyString>,

        /// Another `FileObject` or `FileSet` that this one is contained in, e.g.,
        /// in the case of a file extracted from an archive. When this property
        /// is present, the `contentUrl` is evaluated as a relative path within
        /// the container object. A `DataSource` can also be used in case the data
        /// needs to be filtered or transformed.
        #[serde(
            skip_serializing_if = "Vec::is_empty",
            default,
            deserialize_with = "one_or_many"
        )]
        pub contained_in: Vec<Ref>,

        pub md5: Option<NonEmptyString>,
    }
);

config_struct!(
    #[derive(Config)]
    pub struct FileSet {
        #[serde(rename = "@id")]
        pub id: Id,

        /// The name of the file. As much as possible, the name should reflect
        /// the name of the file as downloaded, including the file extension,
        /// e.g. "images.zip".
        pub name: Option<NonEmptyString>,

        /// The formats of the file, given as a mime type. Unregistered or niche
        /// encoding and file formats can be indicated instead via the most
        /// appropriate URL, e.g. a defining Web page or a Wikipedia/Wikidata
        /// entry.
        pub encoding_format: Option<Iri>,

        /// Another `FileObject` or `FileSet` that this one is contained in, e.g.,
        /// in the case of a file extracted from an archive. When this property
        /// is present, the `contentUrl` is evaluated as a relative path within
        /// the container object. A `DataSource` can also be used in case the data
        /// needs to be filtered or transformed.
        #[serde(
            skip_serializing_if = "Vec::is_empty",
            default,
            deserialize_with = "one_or_many"
        )]
        #[setting(validate = schematic::validate::min_length(1))]
        pub contained_in: Vec<Ref>,

        /// A glob pattern that specifies the files to include.
        #[serde(
            skip_serializing_if = "Vec::is_empty",
            default,
            deserialize_with = "one_or_many"
        )]
        #[setting(validate = schematic::validate::min_length(1))]
        pub includes: Vec<NonEmptyString>,

        /// A glob pattern that specifies the files to exclude.
        #[serde(
            skip_serializing_if = "Vec::is_empty",
            default,
            deserialize_with = "one_or_many"
        )]
        pub excludes: Vec<NonEmptyString>,

        /// A description of the item.
        pub description: Option<NonEmptyString>,
    }
);
