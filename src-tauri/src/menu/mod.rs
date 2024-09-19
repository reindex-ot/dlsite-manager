mod fetch_new_products;
mod refresh_products_all;
mod scan_downloaded_products;

use self::{
    fetch_new_products::fetch_new_products, refresh_products_all::refresh_products_all,
    scan_downloaded_products::scan_downloaded_products,
};
use crate::{
    application::use_application,
    window::{AccountManagementWindow, BuildableWindow, SettingWindow},
};
use anyhow::Error as AnyError;
use tauri::{
    async_runtime::spawn,
    menu::{Menu, MenuEvent, SubmenuBuilder},
    Manager, Runtime,
};
use tauri_plugin_shell::ShellExt;

pub fn create_menu<R: Runtime>(manager: &impl Manager<R>) -> Result<Menu<R>, tauri::Error> {
    let menu = Menu::new(manager)?;

    menu.append(
        &SubmenuBuilder::new(manager, "ウィンドウ")
            .fullscreen()
            .minimize()
            .maximize()
            .close_window()
            .separator()
            .quit()
            .build()?,
    )?;
    menu.append(
        &SubmenuBuilder::new(manager, "編集")
            .undo()
            .redo()
            .cut()
            .copy()
            .paste()
            .select_all()
            .build()?,
    )?;
    menu.append(
        &SubmenuBuilder::new(manager, "アカウント")
            .text("account/open-account-management", "アカウントの管理を開く")
            .build()?,
    )?;
    menu.append(
        &SubmenuBuilder::new(manager, "商品")
            .text("product/fetch-new-products", "新しい商品を取得")
            .text(
                "product/scan-downloaded-products",
                "ダウンロード済みの商品をスキャン",
            )
            .separator()
            .text(
                "product/refresh-products-all",
                "すべての商品をスキャン (ドロップキャッシュ)",
            )
            .build()?,
    )?;
    menu.append(
        &SubmenuBuilder::new(manager, "設定")
            .text("setting/open-setting", "設定を開く")
            .build()?,
    )?;
    menu.append(
        &SubmenuBuilder::new(manager, "ログ")
            .text("log/open-log-directory", "ログのディレクトリを開く")
            .build()?,
    )?;

    Ok(menu)
}

pub fn handle_menu(event: MenuEvent) -> Result<(), AnyError> {
    match event.id.as_ref() {
        "account/open-account-management" => {
            AccountManagementWindow.build_or_focus(use_application().app_handle())?;
        }
        "product/fetch-new-products" => {
            spawn((|| async {
                {
                    let mut is_updating_product = use_application().is_updating_product();

                    if *is_updating_product {
                        return ();
                    }

                    *is_updating_product = true;
                }

                let result = fetch_new_products().await;
                *use_application().is_updating_product() = false;

                result.unwrap();
            })());
        }
        "product/refresh-products-all" => {
            spawn((|| async {
                {
                    let mut is_updating_product = use_application().is_updating_product();

                    if *is_updating_product {
                        return ();
                    }

                    *is_updating_product = true;
                }

                let result = refresh_products_all().await;
                *use_application().is_updating_product() = false;

                result.unwrap();
            })());
        }
        "product/scan-downloaded-products" => {
            spawn((|| async {
                {
                    let mut is_updating_product = use_application().is_updating_product();

                    if *is_updating_product {
                        return ();
                    }

                    *is_updating_product = true;
                }

                let result = scan_downloaded_products().await;
                *use_application().is_updating_product() = false;

                result.unwrap();
            })());
        }
        "setting/open-setting" => {
            SettingWindow.build_or_focus(use_application().app_handle())?;
        }
        "log/open-log-directory" => {
            let app_handle = use_application().app_handle();

            if let Ok(dir) = app_handle.path().app_log_dir() {
                app_handle.shell().open(dir.to_str().unwrap(), None)?;
            }
        }
        _ => {}
    }

    Ok(())
}
