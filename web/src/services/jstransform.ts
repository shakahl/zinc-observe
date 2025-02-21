// Copyright 2022 Zinc Labs Inc. and Contributors

//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at

//      http:www.apache.org/licenses/LICENSE-2.0

//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.

import http from "./http";

const jstransform = {
  list: (
    page_num: number,
    page_size: number,
    sort_by: string,
    desc: boolean,
    name: string,
    org_identifier: string
  ) => {
    return http().get(
      `/api/${org_identifier}/functions?page_num=${page_num}&page_size=${page_size}&sort_by=${sort_by}&desc=${desc}&name=${name}`
    );
  },
  create: (org_identifier: string, data: any) => {
    return http().post(`/api/${org_identifier}/functions`, data);
  },
  update: (org_identifier: string, data: any) => {
    return http().put(`/api/${org_identifier}/functions/${data.name}`, data);
  },
  create_with_index: (
    org_identifier: string,
    stream_name: string,
    data: any
  ) => {
    return http().post(
      `/api/${org_identifier}/${stream_name}/functions/${data.name}?type=${data.stream_type}`,
      data
    );
  },
  delete: (org_identifier: string, transform_name: string) => {
    return http().delete(`/api/${org_identifier}/functions/${transform_name}`);
  },
  delete_stream_function: (
    org_identifier: string,
    stream_name: string,
    transform_name: string
  ) => {
    return http().delete(
      `/api/${org_identifier}/${stream_name}/functions/${transform_name}`
    );
  },
  stream_function: (org_identifier: string, stream_name: string) => {
    return http().get(`/api/${org_identifier}/${stream_name}/functions`);
  },
  apply_stream_function: (
    org_identifier: string,
    stream_name: string,
    function_name: string,
    data: any
  ) => {
    return http().post(
      `/api/${org_identifier}/${stream_name}/functions/${function_name}`,
      data
    );
  },
  remove_stream_function: (
    org_identifier: string,
    stream_name: string,
    function_name: string
  ) => {
    return http().delete(
      `/api/${org_identifier}/${stream_name}/functions/${function_name}`
    );
  },
  create_lookup_table: (
    org_identifier: string,
    table_name: string,
    data: any
  ) => {
    return http({ headers: { "Content-Type": "multipart/form-data" } }).post(
      `/api/${org_identifier}/metadata/${table_name}`,
      data
    );
  },
};

export default jstransform;
