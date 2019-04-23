extern crate bindgen;
extern crate cc;
extern crate glob;
extern crate llvm_tools;

use glob::glob;
use llvm_tools::LlvmTools;
use std::env;
use std::fmt;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

const STDLIB_INCLUDE_PATHS: &[&'static str] = &[
    "/usr/lib/gcc/arm-none-eabi/5.4.1/include",
    "/usr/lib/gcc/arm-none-eabi/5.4.1/include-fixed",
    "/usr/lib/gcc/arm-none-eabi/5.4.1/../../../arm-none-eabi/include",
];

const SDK_INCLUDE_PATHS: &[&'static str] = &[
    "vendor/sdk/component/soc/realtek/common/bsp",
    "vendor/sdk/component/os/freertos",
    "vendor/sdk/component/os/freertos/freertos_v8.1.2/Source/include",
    "vendor/sdk/component/os/freertos/freertos_v8.1.2/Source/portable/GCC/ARM_CM3",
    "vendor/sdk/component/os/os_dep/include",
    "vendor/sdk/component/soc/realtek/8195a/misc/driver",
    "vendor/sdk/component/soc/realtek/8195a/misc/os",
    "vendor/sdk/component/common/api/network/include",
    "vendor/sdk/component/common/api",
    "vendor/sdk/component/common/api/platform",
    "vendor/sdk/component/common/api/wifi",
    "vendor/sdk/component/common/api/wifi/rtw_wpa_supplicant/src",
    "vendor/sdk/component/common/mbed/api",
    "vendor/sdk/component/common/mbed/hal",
    "vendor/sdk/component/common/mbed/hal_ext",
    "vendor/sdk/component/common/mbed/targets/hal/rtl8195a",
    "vendor/sdk/component/common/file_system",
    "vendor/sdk/component/common/network",
    "vendor/sdk/component/common/network/lwip/lwip_v1.4.1/port/realtek/freertos",
    "vendor/sdk/component/common/network/lwip/lwip_v1.4.1/src/include",
    "vendor/sdk/component/common/network/lwip/lwip_v1.4.1/src/include/lwip",
    "vendor/sdk/component/common/network/lwip/lwip_v1.4.1/src/include/ipv4",
    "vendor/sdk/component/common/network/lwip/lwip_v1.4.1/port/realtek",
    "vendor/sdk/component/common/test",
    "vendor/sdk/component/soc/realtek/8195a/cmsis",
    "vendor/sdk/component/soc/realtek/8195a/cmsis/device",
    "vendor/sdk/component/soc/realtek/8195a/fwlib",
    "vendor/sdk/component/soc/realtek/8195a/fwlib/rtl8195a",
    "vendor/sdk/component/soc/realtek/8195a/misc/platform",
    "vendor/sdk/component/soc/realtek/8195a/misc/rtl_std_lib/include",
    "vendor/sdk/component/common/drivers/wlan/realtek/include",
    "vendor/sdk/component/common/drivers/wlan/realtek/src/osdep",
    "vendor/sdk/component/common/drivers/wlan/realtek/src/hci",
    "vendor/sdk/component/common/drivers/wlan/realtek/src/hal",
    "vendor/sdk/component/common/drivers/wlan/realtek/src/hal/OUTSRC",
    "vendor/sdk/component/soc/realtek/8195a/fwlib/ram_lib/wlan/realtek/wlan_ram_map/rom",
    "vendor/sdk/component/common/network/ssl/polarssl-1.3.8/include",
    "vendor/sdk/component/common/network/ssl/ssl_ram_map/rom",
    "vendor/sdk/component/common/utilities",
    "vendor/sdk/component/soc/realtek/8195a/misc/rtl_std_lib/include",
    "vendor/sdk/component/common/application/apple/WACServer/External/Curve25519",
    "vendor/sdk/component/common/application/apple/WACServer/External/GladmanAES",
    "vendor/sdk/component/soc/realtek/8195a/fwlib/ram_lib/usb_otg/include",
    "vendor/sdk/component/common/video/v4l2/inc",
    "vendor/sdk/component/common/media/rtp_codec",
    "vendor/sdk/component/common/drivers/usb_class/host/uvc/inc",
    "vendor/sdk/component/common/drivers/usb_class/device",
    "vendor/sdk/component/common/drivers/usb_class/device/class",
    "vendor/sdk/component/common/drivers/sdio/realtek/sdio_host/inc",
    "vendor/sdk/component/common/audio",
    "vendor/sdk/component/common/drivers/i2s",
    "vendor/sdk/component/common/application/xmodem",
    "vendor/sdk/component/common/application/mqtt/MQTTClient",
];

const SDK_C_FILES: &[&'static str] = &[
    //cmsis
    "vendor/sdk/component/soc/realtek/8195a/cmsis/device/system_8195a.c",
    //console
    "vendor/sdk/component/common/api/at_cmd/atcmd_cloud.c",
    "vendor/sdk/component/common/api/at_cmd/atcmd_ethernet.c",
    "vendor/sdk/component/common/api/at_cmd/atcmd_lwip.c",
    "vendor/sdk/component/common/api/at_cmd/atcmd_sys.c",
    "vendor/sdk/component/common/api/at_cmd/atcmd_wifi.c",
    "vendor/sdk/component/common/api/at_cmd/log_service.c",
    "vendor/sdk/component/soc/realtek/8195a/misc/driver/low_level_io.c",
    "vendor/sdk/component/soc/realtek/8195a/misc/driver/rtl_consol.c",
    //network - api
    "vendor/sdk/component/common/api/wifi/rtw_wpa_supplicant/src/crypto/tls_polarssl.c",
    "vendor/sdk/component/common/api/wifi/rtw_wpa_supplicant/wpa_supplicant/wifi_eap_config.c",
    "vendor/sdk/component/common/api/wifi/rtw_wpa_supplicant/wpa_supplicant/wifi_p2p_config.c",
    "vendor/sdk/component/common/api/wifi/rtw_wpa_supplicant/wpa_supplicant/wifi_wps_config.c",
    "vendor/sdk/component/common/api/wifi/wifi_conf.c",
    "vendor/sdk/component/common/api/wifi/wifi_ind.c",
    "vendor/sdk/component/common/api/wifi/wifi_promisc.c",
    "vendor/sdk/component/common/api/wifi/wifi_simple_config.c",
    "vendor/sdk/component/common/api/wifi/wifi_util.c",
    "vendor/sdk/component/common/api/lwip_netconf.c",
    //network - app
    "vendor/sdk/component/common/application/mqtt/MQTTClient/MQTTClient.c",
    "vendor/sdk/component/common/application/mqtt/MQTTPacket/MQTTConnectClient.c",
    "vendor/sdk/component/common/application/mqtt/MQTTPacket/MQTTConnectServer.c",
    "vendor/sdk/component/common/application/mqtt/MQTTPacket/MQTTDeserializePublish.c",
    "vendor/sdk/component/common/application/mqtt/MQTTPacket/MQTTFormat.c",
    "vendor/sdk/component/common/application/mqtt/MQTTClient/MQTTFreertos.c",
    "vendor/sdk/component/common/application/mqtt/MQTTPacket/MQTTPacket.c",
    "vendor/sdk/component/common/application/mqtt/MQTTPacket/MQTTSerializePublish.c",
    "vendor/sdk/component/common/application/mqtt/MQTTPacket/MQTTSubscribeClient.c",
    "vendor/sdk/component/common/application/mqtt/MQTTPacket/MQTTSubscribeServer.c",
    "vendor/sdk/component/common/application/mqtt/MQTTPacket/MQTTUnsubscribeClient.c",
    "vendor/sdk/component/common/application/mqtt/MQTTPacket/MQTTUnsubscribeServer.c",
    "vendor/sdk/component/soc/realtek/8195a/misc/platform/ota_8195a.c",
    "vendor/sdk/component/common/api/network/src/ping_test.c",
    "vendor/sdk/component/common/utilities/ssl_client.c",
    "vendor/sdk/component/common/utilities/ssl_client_ext.c",
    "vendor/sdk/component/common/utilities/tcptest.c",
    "vendor/sdk/component/common/api/network/src/wlan_network.c",
    //network - lwip
    "vendor/sdk/component/common/network/lwip/lwip_v1.4.1/src/api/api_lib.c",
    "vendor/sdk/component/common/network/lwip/lwip_v1.4.1/src/api/api_msg.c",
    "vendor/sdk/component/common/network/lwip/lwip_v1.4.1/src/api/err.c",
    "vendor/sdk/component/common/network/lwip/lwip_v1.4.1/src/api/netbuf.c",
    "vendor/sdk/component/common/network/lwip/lwip_v1.4.1/src/api/netdb.c",
    "vendor/sdk/component/common/network/lwip/lwip_v1.4.1/src/api/netifapi.c",
    "vendor/sdk/component/common/network/lwip/lwip_v1.4.1/src/api/sockets.c",
    "vendor/sdk/component/common/network/lwip/lwip_v1.4.1/src/api/tcpip.c",
    "vendor/sdk/component/common/network/lwip/lwip_v1.4.1/src/core/ipv4/autoip.c",
    "vendor/sdk/component/common/network/lwip/lwip_v1.4.1/src/core/ipv4/icmp.c",
    "vendor/sdk/component/common/network/lwip/lwip_v1.4.1/src/core/ipv4/igmp.c",
    "vendor/sdk/component/common/network/lwip/lwip_v1.4.1/src/core/ipv4/inet.c",
    "vendor/sdk/component/common/network/lwip/lwip_v1.4.1/src/core/ipv4/inet_chksum.c",
    "vendor/sdk/component/common/network/lwip/lwip_v1.4.1/src/core/ipv4/ip.c",
    "vendor/sdk/component/common/network/lwip/lwip_v1.4.1/src/core/ipv4/ip_addr.c",
    "vendor/sdk/component/common/network/lwip/lwip_v1.4.1/src/core/ipv4/ip_frag.c",
    "vendor/sdk/component/common/network/lwip/lwip_v1.4.1/src/core/def.c",
    "vendor/sdk/component/common/network/lwip/lwip_v1.4.1/src/core/dhcp.c",
    "vendor/sdk/component/common/network/lwip/lwip_v1.4.1/src/core/dns.c",
    "vendor/sdk/component/common/network/lwip/lwip_v1.4.1/src/core/init.c",
    "vendor/sdk/component/common/network/lwip/lwip_v1.4.1/src/core/lwip_timers.c",
    "vendor/sdk/component/common/network/lwip/lwip_v1.4.1/src/core/mem.c",
    "vendor/sdk/component/common/network/lwip/lwip_v1.4.1/src/core/memp.c",
    "vendor/sdk/component/common/network/lwip/lwip_v1.4.1/src/core/netif.c",
    "vendor/sdk/component/common/network/lwip/lwip_v1.4.1/src/core/pbuf.c",
    "vendor/sdk/component/common/network/lwip/lwip_v1.4.1/src/core/raw.c",
    "vendor/sdk/component/common/network/lwip/lwip_v1.4.1/src/core/stats.c",
    "vendor/sdk/component/common/network/lwip/lwip_v1.4.1/src/core/sys.c",
    "vendor/sdk/component/common/network/lwip/lwip_v1.4.1/src/core/tcp.c",
    "vendor/sdk/component/common/network/lwip/lwip_v1.4.1/src/core/tcp_in.c",
    "vendor/sdk/component/common/network/lwip/lwip_v1.4.1/src/core/tcp_out.c",
    "vendor/sdk/component/common/network/lwip/lwip_v1.4.1/src/core/udp.c",
    "vendor/sdk/component/common/network/lwip/lwip_v1.4.1/src/netif/etharp.c",
    "vendor/sdk/component/common/network/lwip/lwip_v1.4.1/port/realtek/freertos/ethernetif.c",
    "vendor/sdk/component/common/drivers/wlan/realtek/src/osdep/lwip_intf.c",
    "vendor/sdk/component/common/network/lwip/lwip_v1.4.1/port/realtek/freertos/sys_arch.c",
    "vendor/sdk/component/common/network/dhcp/dhcps.c",
    "vendor/sdk/component/common/network/sntp/sntp.c",
    //network - httpc
    "vendor/sdk/component/common/network/httpc/httpc_tls.c",
    //network - httpd
    "vendor/sdk/component/common/network/httpd/httpd_tls.c",
    //network - mdns
    "vendor/sdk/component/common/network/mDNS/mDNSPlatform.c",
    //network - wsclient
    "vendor/sdk/component/common/network/websocket/wsclient_tls.c",
    //os - freertos
    "vendor/sdk/component/os/freertos/freertos_v8.1.2/Source/portable/MemMang/heap_5.c",
    "vendor/sdk/component/os/freertos/freertos_v8.1.2/Source/portable/GCC/ARM_CM3/port.c",
    "vendor/sdk/component/os/freertos/cmsis_os.c",
    "vendor/sdk/component/os/freertos/freertos_v8.1.2/Source/croutine.c",
    "vendor/sdk/component/os/freertos/freertos_v8.1.2/Source/event_groups.c",
    "vendor/sdk/component/os/freertos/freertos_v8.1.2/Source/list.c",
    "vendor/sdk/component/os/freertos/freertos_v8.1.2/Source/queue.c",
    "vendor/sdk/component/os/freertos/freertos_v8.1.2/Source/tasks.c",
    "vendor/sdk/component/os/freertos/freertos_v8.1.2/Source/timers.c",
    //os - osdep
    "vendor/sdk/component/os/os_dep/device_lock.c",
    "vendor/sdk/component/os/freertos/freertos_service.c",
    "vendor/sdk/component/soc/realtek/8195a/misc/os/mailbox.c",
    "vendor/sdk/component/soc/realtek/8195a/misc/os/osdep_api.c",
    "vendor/sdk/component/os/os_dep/osdep_service.c",
    "vendor/sdk/component/os/os_dep/tcm_heap.c",
    //peripheral - api
    "vendor/sdk/component/common/mbed/targets/hal/rtl8195a/analogin_api.c",
    "vendor/sdk/component/common/mbed/targets/hal/rtl8195a/dma_api.c",
    "vendor/sdk/component/common/mbed/targets/hal/rtl8195a/efuse_api.c",
    "vendor/sdk/component/common/mbed/targets/hal/rtl8195a/ethernet_api.c",
    "vendor/sdk/component/common/drivers/ethernet_mii/ethernet_mii.c",
    "vendor/sdk/component/common/mbed/targets/hal/rtl8195a/flash_api.c",
    "vendor/sdk/component/common/mbed/targets/hal/rtl8195a/gpio_api.c",
    "vendor/sdk/component/common/mbed/targets/hal/rtl8195a/gpio_irq_api.c",
    "vendor/sdk/component/common/mbed/targets/hal/rtl8195a/i2c_api.c",
    "vendor/sdk/component/common/mbed/targets/hal/rtl8195a/i2s_api.c",
    "vendor/sdk/component/common/mbed/targets/hal/rtl8195a/log_uart_api.c",
    "vendor/sdk/component/common/mbed/targets/hal/rtl8195a/nfc_api.c",
    "vendor/sdk/component/common/mbed/targets/hal/rtl8195a/pinmap.c",
    "vendor/sdk/component/common/mbed/targets/hal/rtl8195a/pinmap_common.c",
    "vendor/sdk/component/common/mbed/targets/hal/rtl8195a/port_api.c",
    "vendor/sdk/component/common/mbed/targets/hal/rtl8195a/pwmout_api.c",
    "vendor/sdk/component/common/mbed/targets/hal/rtl8195a/rtc_api.c",
    "vendor/sdk/component/common/mbed/targets/hal/rtl8195a/serial_api.c",
    "vendor/sdk/component/common/mbed/targets/hal/rtl8195a/sleep.c",
    "vendor/sdk/component/common/mbed/targets/hal/rtl8195a/spdio_api.c",
    "vendor/sdk/component/common/mbed/targets/hal/rtl8195a/spi_api.c",
    "vendor/sdk/component/common/mbed/targets/hal/rtl8195a/sys_api.c",
    "vendor/sdk/component/common/mbed/targets/hal/rtl8195a/timer_api.c",
    "vendor/sdk/component/common/mbed/targets/hal/rtl8195a/us_ticker.c",
    "vendor/sdk/component/common/mbed/common/us_ticker_api.c",
    "vendor/sdk/component/common/mbed/common/wait_api.c",
    "vendor/sdk/component/common/mbed/targets/hal/rtl8195a/wdt_api.c",
    //peripheral - hal
    "vendor/sdk/component/soc/realtek/8195a/fwlib/src/hal_32k.c",
    "vendor/sdk/component/soc/realtek/8195a/fwlib/src/hal_adc.c",
    "vendor/sdk/component/soc/realtek/8195a/fwlib/src/hal_gdma.c",
    "vendor/sdk/component/soc/realtek/8195a/fwlib/src/hal_gpio.c",
    "vendor/sdk/component/soc/realtek/8195a/fwlib/src/hal_i2c.c",
    "vendor/sdk/component/soc/realtek/8195a/fwlib/src/hal_i2s.c",
    "vendor/sdk/component/soc/realtek/8195a/fwlib/src/hal_mii.c",
    "vendor/sdk/component/soc/realtek/8195a/fwlib/src/hal_nfc.c",
    "vendor/sdk/component/soc/realtek/8195a/fwlib/src/hal_pcm.c",
    "vendor/sdk/component/soc/realtek/8195a/fwlib/src/hal_pwm.c",
    "vendor/sdk/component/soc/realtek/8195a/fwlib/src/hal_sdr_controller.c",
    "vendor/sdk/component/soc/realtek/8195a/fwlib/src/hal_ssi.c",
    "vendor/sdk/component/soc/realtek/8195a/fwlib/src/hal_timer.c",
    "vendor/sdk/component/soc/realtek/8195a/fwlib/src/hal_uart.c",
    //peripheral - rtl8195a
    "vendor/sdk/component/soc/realtek/8195a/fwlib/rtl8195a/src/rtl8195a_adc.c",
    "vendor/sdk/component/soc/realtek/8195a/fwlib/rtl8195a/src/rtl8195a_gdma.c",
    "vendor/sdk/component/soc/realtek/8195a/fwlib/rtl8195a/src/rtl8195a_gpio.c",
    "vendor/sdk/component/soc/realtek/8195a/fwlib/rtl8195a/src/rtl8195a_i2c.c",
    "vendor/sdk/component/soc/realtek/8195a/fwlib/rtl8195a/src/rtl8195a_i2s.c",
    "vendor/sdk/component/soc/realtek/8195a/fwlib/rtl8195a/src/rtl8195a_mii.c",
    "vendor/sdk/component/soc/realtek/8195a/fwlib/rtl8195a/src/rtl8195a_nfc.c",
    "vendor/sdk/component/soc/realtek/8195a/fwlib/rtl8195a/src/rtl8195a_pwm.c",
    "vendor/sdk/component/soc/realtek/8195a/fwlib/rtl8195a/src/rtl8195a_ssi.c",
    "vendor/sdk/component/soc/realtek/8195a/fwlib/rtl8195a/src/rtl8195a_timer.c",
    "vendor/sdk/component/soc/realtek/8195a/fwlib/rtl8195a/src/rtl8195a_uart.c",
    //all:SRC_C += vendor/sdk/component/common/drivers/wlan/realtek/src/core/option/rtw_opt_skbuf.c
    "vendor/sdk/component/common/utilities/cJSON.c",
    "vendor/sdk/component/common/utilities/http_client.c",
    "vendor/sdk/component/common/utilities/uart_socket.c",
    "vendor/sdk/component/common/utilities/webserver.c",
    "vendor/sdk/component/common/utilities/xml.c",
];

const SDK_STATIC_LIBS: &[&'static str] = &[
    "platform",
    "wlan",
    "http",
    "dct",
    "wps",
    "rtlstd",
    "websocket",
    "xmodem",
    "mdns"
];

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let sdk_staticlib_path = PathBuf::from("vendor/sdk/component/soc/realtek/8195a/misc/bsp/lib/common/GCC").canonicalize().unwrap();

    let tools = LlvmTools::new().unwrap();
    let objcopy = match tools.tool("llvm-objcopy") {
        Some(path) => path,
        None => panic!("Couldn't find objcopy"),
    };

    for lib_name in SDK_STATIC_LIBS.iter() {
        fs::create_dir_all(out_path.join("static").join(lib_name)).expect("Could not create directory");

        Command::new("ar")
            .arg("x")
            .arg(sdk_staticlib_path.join(format!("lib_{}.a", lib_name)).to_str().unwrap())
            .current_dir(out_path.join("static").join(lib_name))
            .status().unwrap();
    }

    let sdk_static_lib_object_paths = glob(out_path.join("static/**/*.o").to_str().unwrap()).unwrap().into_iter().filter_map(|entry| {
        match entry {
            Ok(path) => Some(path),
            Err(_) => None,
        }
    }).collect::<Vec<_>>();

    Command::new(objcopy)
        .args(&["--rename-section", ".data=.loader.data,contents,alloc,load,readonly,data"])
        .args(&["-I", "binary"])
        .args(&["-B", "arm"])
        .arg("vendor/sdk/component/soc/realtek/8195a/misc/bsp/image/ram_1.r.bin")
        .arg(out_path.join("ram_1.r.o"))
        .status().unwrap();

    let mut compiler = cc::Build::new();

    compiler
        .include("include")
        .define("M3", None)
        .define("CONFIG_PLATFORM_8195A", None)
        .define("GCC_ARMCM3", None)
        .define("F_CPU", "166000000L")
        .flag("-mcpu=cortex-m3")
        .flag("-mthumb")
        .flag("-g2")
        .flag("-w")
        .flag("-O2")
        .flag("-Wno-pointer-sign")
        .flag("-fno-common")
        .flag("-fmessage-length=0")
        .flag("-ffunction-sections")
        .flag("-fdata-sections")
        .flag("-fomit-frame-pointer")
        .flag("-fno-short-enums")
        .flag("-std=gnu99")
        .flag("-fsigned-char");

    for path in STDLIB_INCLUDE_PATHS {
        compiler.include(path);
    }

    for path in SDK_INCLUDE_PATHS {
        compiler.include(path);
    }

    for path in sdk_static_lib_object_paths {
        compiler.object(path);
    }

    compiler
        .object(out_path.join("ram_1.r.o"))
        .object("vendor/sdk/component/soc/realtek/8195a/misc/bsp/lib/common/GCC/lib_wlan.a")
        .files(SDK_C_FILES)
        .file("src/freertos_rs.c")
        .compile("sdk");

    let bindings = bindgen::Builder::default()
        .header("include/wrapper.h")
        .whitelist_function("wifi_manager_init")
        .whitelist_function("wifi_off")
        .whitelist_function("wifi_on")
        .whitelist_function("wifi_scan_networks")
        .whitelist_function("vTaskStartScheduler")
        .whitelist_function("pvPortMalloc")
        .whitelist_function("vPortFree")
        .whitelist_type("rtw_mode_t")
        .whitelist_type("rtw_scan_result_t")
        .whitelist_type("rtw_scan_result_handler_t")
        .clang_arg("-Iinclude")
        .clang_args(STDLIB_INCLUDE_PATHS.iter().map(|path| format!("-I{}", path)))
        .clang_args(SDK_INCLUDE_PATHS.iter().map(|path| format!("-I{}", path)))
        .use_core()
        .ctypes_prefix("cty")
        .rustfmt_bindings(true)
        .generate()
        .expect("Unable to generate C SDK bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings.rs!");
}
