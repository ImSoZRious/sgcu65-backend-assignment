ปัณณวิชญ์ โลหะนิมิต

# How to start
rust toolchain is required.
## Rust toolchain setup
[Rustup](https://www.rust-lang.org/learn/get-started)

## Compile and run
```sh
cargo run
```

# Troubleshoot
## [Program exit with code 3](https://github.com/diesel-rs/diesel/discussions/2947)

# TODO
- [ ] use functor for api
- [ ] find task by name
- [ ] find user by firstname, lastname
- [ ] assign task to user
- [ ] add role enum
- [ ] add properly do timestamp
- [ ] rewrite api specification
- [ ] Redesign database for team
- [ ] Team (CRUD)
- [ ] Team Find
- [ ] Team assign user
- [ ] Team assign task
- [ ] TBD
- [ ] Docker if have excess time


# SGCU65 Backend Recruitment

แบบทดสอบทางวิศวกรรมซอฟต์แวร์ เพื่อสรรหาบุคคลเข้ารับตำแหน่ง Backend Developer ประจำปีการศึกษา 2565

# สารบัญ

- [โจทย์](#โจทย์)
- [Note](#Note)

## งานของคุณ

เขียน API Service ตาม requirement ดังต่อไปนี้

### รายละเอียดของ Feature (Minimum Requirement)

### User

**รายละเอียดการเก็บข้อมูล minimum อยู่ข้างล่าง**

```markdown
- [x] สามารถเพิ่มพนักงานใหม่เข้าไปในระบบได้ (Create)
- [x] สามารถดูข้อมูลของพนักงานทุกคนได้ (Read)
- [x] สามารถแก้ไขข้อมูลของพนักงานได้ เช่นชื่อ-สกุล ตำแหน่ง และเงินเดือนของพนักงานได้ (Update)
- [x] สามารถลบข้อมูลพนักงานในระบบได้ (Delete)
- [ ] สามารถค้นหาพนักงานโดยใช้ ชื่อ นามสกุล หรือ ตำแหน่งได้
```

### Task

**รายละเอียดการเก็บข้อมูล minimum อยู่ข้างล่าง**

```markdown
- [x] สามารถสร้าง Task ใหม่ได้ (Create)
- [x] สามารถดูข้อมูลของ Task ทั้งหมดได้ (Read)
- [x] สามารถแก้ไข ข้อมูล/status ของ Task ได้ (Update)
- [x] สามารถลบ Task ได้ (Delete)
- [ ] สามารถค้นหา Task ด้วย name หรือ id ได้ (hint: ถ้าใช้ RDBMS สามารถใช้กำหนด relation ของ model ได้)
```

### Other

```markdown
- [ ] สามารถ assign งานให้ user ได้โดยที่ user 1 คนสามารถรับได้หลายอันและ task 1 อันสามารถมีผู้รับผิดชอบได้หลายคน
```

### รายละเอียดของ Feature (Optional)

### Team

**รายละเอียดการเก็บข้อมูล minimum อยู่ข้างล่าง**

```markdown
- สามารถสร้าง Team ใหม่ได้ (Create)
- สามารถดูข้อมูลของ Team ทั้งหมดได้ (Read)
- สามารถแก้ไข ข้อมูล ของ Team ได้ (Update)
- สามารถลบ Team ได้ (Delete)
- สามารถค้นหา Team ด้วยชื่อ หรือ id ได้
- สามารถ assign User เข้าทีมได้
- สามารถ assign Task ให้กับ team ได้
  - เปลี่ยนจากการ assign task ให้ user เป็นการ assign task ให้ team แทน
```

### Authentication

```markdown
- [ ] สามารถเข้าสู่ระบบได้ (โดยใช้ email, รหัสผ่าน)
```

### Authorization

```markdown
- [ ] สามารถแบ่งแยก user ออกเป็น 2 role คือ User กับ Admin - User
- [ ] สามารถ login ด้วย username และ password ได้ 
- [ ] สามารถแก้ไข password ของตนเองได้ 
- [ ] สามารถดูข้อมูลของตนเองได้ (นั่นคือ API มีวิธีระบุตัวตนว่าใครเป็นคนยิง API) 
- [x] **ไม่สามารถ** Create Update Delete Read ได้ - Admin - ทำสิ่งที่ employee ทำได้ - สามารถ สิ่งที่ระบุไว้ 5 ข้อด้านบนได้ (ให้เฉพาะ HR ใช้งานเท่านั้น)
```

## การเก็บข้อมูล (Minimum)

ในส่วนของการเก็บข้อมูลนี้คือ minimum requirement ที่จะต้องเก็บเข้าสู่ระบบถ้ามีเพิ่มเติมมามากกว่านี้สามารถใส่ได้เลย

User

```markdown
- [x] email
- [x] firstname
- [x] surname
- [x] role
```

Task

```markdown
- [x] name
- [x] content
- [x] status
- [x] deadline
```

Team

```markdown
- [ ] name
```