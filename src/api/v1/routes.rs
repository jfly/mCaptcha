/*
 * Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
use actix_auth_middleware::GetLoginRoute;

use super::account::routes::Account;
use super::auth::routes::Auth;
use super::mcaptcha::routes::Captcha;
use super::meta::routes::Meta;
use super::notifications::routes::Notifications;
use super::pow::routes::PoW;
use super::survey::routes::Survey;

pub const ROUTES: Routes = Routes::new();

pub struct Routes {
    pub auth: Auth,
    pub account: Account,
    pub captcha: Captcha,
    pub meta: Meta,
    pub pow: PoW,
    pub survey: Survey,
    pub notifications: Notifications,
}

impl Routes {
    const fn new() -> Routes {
        Routes {
            auth: Auth::new(),
            account: Account::new(),
            captcha: Captcha::new(),
            meta: Meta::new(),
            pow: PoW::new(),
            notifications: Notifications::new(),
            survey: Survey::new(),
        }
    }
}

impl GetLoginRoute for Routes {
    fn get_login_route(&self, src: Option<&str>) -> String {
        self.auth.get_login_route(src)
    }
}
