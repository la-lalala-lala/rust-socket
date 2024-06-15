use crate::domain::dto::page::ExtendPageDTO;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Page<T> {
    /// 当前页面的数据
    pub records: Option<Vec<T>>,
    /// 总数据行
    pub total_row: u64,
    /// 总页数
    pub total_page: u64,
    /// 当前页
    pub page_no: u64,
    /// 页宽
    pub page_size: u64,
}

impl<T> Page<T> {
    /// 在计算总页数后，判断请求的页宽是否在范围内
    pub fn set_page_now(&self) -> u64 {
        if self.page_no >= 1 && self.page_no <= self.total_page {
            //在范围内
            self.page_no
        } else {
            //不在范围内
            1
        }
    }

    /// 计算总页数
    pub fn set_total_page(&self) -> u64 {
        if self.total_row % self.page_size == 0 {
            self.total_row / self.page_size
        } else {
            self.total_row / self.page_size + 1
        }
    }

    pub fn page_query(total_row: u64, extend_param: &ExtendPageDTO) -> Page<T> {
        // 构造返回结果，并初始化总记录数
        let mut result = Page::<T> {
            records: None,
            total_row,
            total_page: 0,
            page_no: if extend_param.page_no.is_some() {
                extend_param.page_no.unwrap()
            } else {
                1
            },
            page_size: if extend_param.page_size.is_some() {
                extend_param.page_size.unwrap()
            } else {
                10
            },
        };
        //计算总页数
        result.total_page = result.set_total_page();
        //设置当前的页码-并校验是否超出页码范围
        result.page_no = result.set_page_now();
        result
    }
}