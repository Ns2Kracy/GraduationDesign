# GraduationDesign

## 项目名称：农产品质量安全管理系统的设计与实现

## 模块划分

1. **种植管理模块**(Plantation Management Module)
2. **质量检测模块**(Quality Inspection Module)
3. **风险评估与预警模块**(Risk Assessment and Alert Module)
4. **信息报告与分析模块**(Information Reporting and Analysis Module)
5. **系统管理与权限控制模块**(System Management and Access Control Module)
6. **执法检查模块**(Enforcement Inspection Module)
7. **统计查询模块**(Statistical Query Module)
8. **质量反馈与投诉模块**(Quality Feedback and Complaint Module)

## 数据库设计

### 种植管理模块

**农场信息表(Farms Table)**

| 字段      | 数据类型           | 描述                     |
| --------- | ------------------ | ------------------------ |
| farm_id   | SERIAL PRIMARY KEY | 农场唯一标识符           |
| farm_name | VARCHAR(100)       | 农场名称                 |
| location  | VARCHAR(200)       | 农场位置                 |
| area      | FLOAT              | 农场面积(单位：平方公里) |

**种子表(Seeds Table)**

| 字段       | 数据类型           | 描述           |
| ---------- | ------------------ | -------------- |
| seed_id    | SERIAL PRIMARY KEY | 种子唯一标识符 |
| seed_name  | VARCHAR(100)       | 种子名称       |
| supplier   | VARCHAR(100)       | 供应商名称     |
| shelf_life | INTEGER            | 保质期(月)     |
| quantity   | INTEGER            | 库存数量       |
| remark     | TEXT               | 备注           |

**种植记录表(Planting Records Table)**

| 字段               | 数据类型           | 描述                                                |
| ------------------ | ------------------ | --------------------------------------------------- |
| planting_record_id | SERIAL PRIMARY KEY | 种植记录唯一标识符                                  |
| farm_id            | INTEGER            | 外键，农场ID Foreign key referencing farms(farm_id) |
| seed_id            | INTEGER            | 外键，种子ID Foreign key referencing seeds(seed_id) |
| crop_name          | VARCHAR(100)       | 农作物名称                                          |
| planting_date      | DATE               | 种植日期                                            |
| planting_area      | FLOAT              | 种植面积(单位：亩)                                  |
| remark             | TEXT               | 备注                                                |

**农药使用记录表(Pesticide Usage Records Table)**

| 字段                    | 数据类型           | 描述                                                |
| ----------------------- | ------------------ | --------------------------------------------------- |
| pesticide_usage_id      | SERIAL PRIMARY KEY | 使用记录唯一标识符                                  |
| farm_id                 | INTEGER            | 外键，农场ID Foreign key referencing farms(farm_id) |
| pesticide_name          | VARCHAR(100)       | 农药名称                                            |
| pesticide_usage_date    | DATE               | 使用日期                                            |
| pesticide_usage_amount  | FLOAT              | 使用量(单位：升)                                    |
| pesticide_usage_purpose | TEXT               | 使用目的                                            |
| remark                  | TEXT               | 备注                                                |

**收获记录表(Harvest Records Table)**

| 字段           | 数据类型           | 描述                                                |
| -------------- | ------------------ | --------------------------------------------------- |
| harvest_id     | SERIAL PRIMARY KEY | 收获记录唯一标识符                                  |
| farm_id        | INTEGER            | 外键，农场ID Foreign key referencing farms(farm_id) |
| crop_name      | VARCHAR(100)       | 农作物名称                                          |
| harvest_date   | DATE               | 收获日期                                            |
| harvest_amount | FLOAT              | 收获数量(单位：吨)                                  |
| remark         | TEXT               | 备注                                                |

### 质量检测模块

**农产品抽检记录表(Product Inspection Records Table)**

| 字段              | 数据类型           | 描述                                                |
| ----------------- | ------------------ | --------------------------------------------------- |
| inspection_id     | SERIAL PRIMARY KEY | 抽检记录唯一标识符                                  |
| farm_id           | INTEGER            | 外键，农场ID Foreign key referencing farms(farm_id) |
| crop_name         | VARCHAR(100)       | 农作物名称                                          |
| inspection_date   | DATE               | 抽检日期                                            |
| inspection_result | VARCHAR(50)        | 抽检结果                                            |
| remark            | TEXT               | 备注                                                |

**农药残留检测记录表(Pesticide Residue Detection Records Table)**

| 字段                 | 数据类型           | 描述                                                |
| -------------------- | ------------------ | --------------------------------------------------- |
| pesticide_residue_id | SERIAL PRIMARY KEY | 残留检测记录唯一标识符                              |
| farm_id              | INTEGER            | 外键，农场ID Foreign key referencing farms(farm_id) |
| crop_name            | VARCHAR(100)       | 农作物名称                                          |
| detection_date       | DATE               | 检测日期                                            |
| pesticide_name       | VARCHAR(100)       | 农药名称                                            |
| residue_level        | FLOAT              | 残留水平(单位：mg/kg)                               |
| detection_method     | TEXT               | 检测方法                                            |
| remark               | TEXT               | 备注                                                |

### 风险评估与预警模块数据库设计

**风险评估表(Risk Assessment Table)**
| 字段            | 数据类型    | 描述                           |
| --------------- | ----------- | ------------------------------ |
| assessment_id   | SERIAL      | 主键，唯一标识符               |
| assessment_type | VARCHAR(50) | 评估类型（自然灾害、病虫害等） |
| assessment_date | DATE        | 评估日期                       |
| risk_level      | VARCHAR(20) | 风险级别（低、中、高）         |
| alert_info      | TEXT        | 预警信息描述                   |

**预警信息表(Alert Information Table)**
| 字段          | 数据类型    | 描述                           |
| ------------- | ----------- | ------------------------------ |
| alert_id      | SERIAL      | 主键，唯一标识符               |
| alert_type    | VARCHAR(50) | 预警类型（自然灾害、病虫害等） |
| alert_date    | DATE        | 预警日期                       |
| alert_content | TEXT        | 预警内容描述                   |
| alert_level   | VARCHAR(20) | 预警级别（低、中、高）         |

### 信息报告与分析模块数据库设计

**报告表(Report Table)**
| 字段           | 数据类型    | 描述                             |
| -------------- | ----------- | -------------------------------- |
| report_id      | SERIAL      | 主键，唯一标识符                 |
| report_date    | DATE        | 报告日期                         |
| report_content | TEXT        | 报告内容描述                     |
| report_type    | VARCHAR(50) | 报告类型（质量报告、生产报告等） |

**分析表(Analysis Table)**
| 字段             | 数据类型    | 描述                             |
| ---------------- | ----------- | -------------------------------- |
| analysis_id      | SERIAL      | 主键，唯一标识符                 |
| analysis_date    | DATE        | 分析日期                         |
| analysis_type    | VARCHAR(50) | 分析类型（质量分析、生产分析等） |
| analysis_content | TEXT        | 分析内容描述                     |
| analysis_result  | TEXT        | 分析结果描述                     |

### 系统管理与权限控制模块

**用户表(Users Table)**

| 字段          | 数据类型           | 描述                         |
| ------------- | ------------------ | ---------------------------- |
| user_id       | SERIAL PRIMARY KEY | 用户唯一标识符               |
| username      | VARCHAR(100)       | 用户名                       |
| password      | VARCHAR(100)       | 密码(加密存储)               |
| role          | VARCHAR(50)        | 用户角色(管理员、普通用户等) |
| created_at    | TIMESTAMP          | 用户创建时间                 |
| updated_at    | TIMESTAMP          | 用户信息更新时间             |
| deleted_at    | TIMESTAMP          | 用户删除时间                 |
| last_login_at | TIMESTAMP          | 用户最后登录时间             |
| status        | VARCHAR(20)        | 用户状态(活跃、禁用等)       |

**操作日志表(Operation Logs Table)**

| 字段        | 数据类型           | 描述                                                |
| ----------- | ------------------ | --------------------------------------------------- |
| log_id      | SERIAL PRIMARY KEY | 日志唯一标识符                                      |
| user_id     | INTEGER            | 外键，用户ID Foreign key referencing users(user_id) |
| log_date    | TIMESTAMP          | 日志记录时间                                        |
| log_content | TEXT               | 日志内容                                            |

### 执法检查模块

**执法检查记录表(Enforcement Records Table)**

| 字段名                | 数据类型           | 描述                                                |
| --------------------- | ------------------ | --------------------------------------------------- |
| enforcement_record_id | SERIAL PRIMARY KEY | 主键，唯一标识符                                    |
| farm_id               | INTEGER            | 外键，农场ID Foreign key referencing farms(farm_id) |
| inspection_date       | TIMESTAMP          | 执法检查的时间                                      |
| inspector_name        | VARCHAR(50)        | 执法检查人员的姓名                                  |
| inspection_content    | TEXT               | 执法检查的具体内容                                  |
| inspection_result     | VARCHAR(20)        | 执法检查的结果(合格、不合格等)                      |
| penalty_info          | TEXT               | 处罚信息                                            |

**处罚记录表(Penalty Records Table)**

| 字段名              | 数据类型       | 描述                                                |
| ------------------- | -------------- | --------------------------------------------------- |
| penalty_record_id   | SERIAL         | 主键，唯一标识符                                    |
| penalty_record_date | TIMESTAMP      | 处罚记录的时间                                      |
| farm_id             | INTEGER        | 外键，农场ID Foreign key referencing farms(farm_id) |
| penalty_type        | VARCHAR(50)    | 处罚的类型(罚款、责令停产等)                        |
| penalty_amount      | DECIMAL(10, 2) | 处罚金额(如有)                                      |
| penalty_reason      | TEXT           | 处罚的具体原因                                      |

### 统计查询模块

**统计数据表(Statistical Data Table)**
| 字段                 | 数据类型       | 描述                                 |
| -------------------- | -------------- | ------------------------------------ |
| statistic_id         | SERIAL         | 主键，唯一标识符                     |
| statistic_date       | DATE           | 统计数据的日期                       |
| statistic_data_type  | VARCHAR(50)    | 数据类型，用于区分不同类型的统计数据 |
| statistic_data_value | DECIMAL(10, 2) | 统计数据的数值                       |
| remark               | TEXT           | 备注                                 |

**查询日志表(Query Logs Table)**
| 字段          | 数据类型  | 描述                                                |
| ------------- | --------- | --------------------------------------------------- |
| query_log_id  | SERIAL    | 主键，唯一标识符                                    |
| user_id       | INTEGER   | 外键，用户ID Foreign key referencing users(user_id) |
| query_date    | TIMESTAMP | 查询日志的时间                                      |
| query_content | TEXT      | 查询内容                                            |

### 质量反馈与投诉模块

**质量反馈表(Quality Feedback Table)**

| 字段             | 数据类型           | 描述                                                |
| ---------------- | ------------------ | --------------------------------------------------- |
| feedback_id      | SERIAL PRIMARY KEY | 反馈唯一标识符                                      |
| farm_id          | INTEGER            | 外键，农场ID Foreign key referencing farms(farm_id) |
| feedback_date    | DATE               | 反馈日期                                            |
| feedback_content | TEXT               | 反馈内容                                            |
| feedback_status  | VARCHAR(20)        | 反馈状态(已处理、未处理等)                          |