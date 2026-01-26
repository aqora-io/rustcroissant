#[macro_export]
macro_rules! config_struct {
    ($impl:item) => {
        #[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        $impl
    };
}

#[macro_export]
macro_rules! config_enum {
    ($impl:item) => {
        #[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        $impl
    };
}

#[macro_export]
macro_rules! config_unit_enum {
    ($impl:item) => {
        $crate::config_enum!(
            #[derive(Copy, Default)]
            $impl
        );
    };
}
