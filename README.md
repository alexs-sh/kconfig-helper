# About

A simple app for comparing Linux kernel configs and showing output as text, CSV
or Markdown tables. In most cases, you don't need this kind of app. A simple
diff is enough. But if development relies on sharing host-specific and "dirty"
configs, then this app could be useful. Here are some examples.

**Markdown**

|# | Parameter | Status | Old Value | New Value |
|--|-----------|--------|-----------|-----------|
|1|[CONFIG_ACPI_FFH](https://cateee.net/lkddb/web-lkddb/ACPI_FFH.html)|Removed|y||
|2|[CONFIG_ADDRESS_MASKING](https://cateee.net/lkddb/web-lkddb/ADDRESS_MASKING.html)|Removed|y||
|3|[CONFIG_ADIN1100_PHY](https://cateee.net/lkddb/web-lkddb/ADIN1100_PHY.html)|Removed|is not set||
|4|[CONFIG_AMD_ATL](https://cateee.net/lkddb/web-lkddb/AMD_ATL.html)|Removed|m||
|5|[CONFIG_AMD_PMF](https://cateee.net/lkddb/web-lkddb/AMD_PMF.html)|Removed|m||
|6|[CONFIG_AMD_PMF_DEBUG](https://cateee.net/lkddb/web-lkddb/AMD_PMF_DEBUG.html)|Removed|is not set||
|7|[CONFIG_ANDROID](https://cateee.net/lkddb/web-lkddb/ANDROID.html)|Added||is not set|


**Text**

```
Modified(CONFIG_WERROR,is not set,y)
Added(CONFIG_WWAN_DEBUGFS,,y)
Modified(CONFIG_WWAN_HWSIM,is not set,m)
Added(CONFIG_X86_AMD_PSTATE_DEFAULT_MODE,,3)
Added(CONFIG_X86_AMD_PSTATE_UT,,m)
--------------------------------------------------------------------------------
SUMMARY
--------------------------------------------------------------------------------
Parameters:
  - Modified:38
  - Added:314
  - Removed:69
  - Unmodified:6985
  - Analyzed:7406
--------------------------------------------------------------------------------

```

# Build and run

```
cargo run --release -- config_before config_after
```

```
cargo run --release -- config_before config_after --format md
```


