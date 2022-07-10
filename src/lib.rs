pub mod number_triangle{
    use rand::Rng;

    pub struct Triangle{
        data: Vec<u8>,
        size: u32
    }

    pub fn create_triangle(size: u32) -> Triangle{
        let mut data = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..(size*(size+1)/2){
            data.push(rng.gen_range(1..10));
        }
        Triangle{
            data,
            size
        }
    }

    pub fn wrap(data: Vec<u8>) -> Triangle{
        Triangle{
            size: ((data.len()*2) as f32).sqrt() as u32,
            data
        }
    }

    pub fn cancel_offset(mut path: Vec<u32>) -> Vec<u32>{
            for i in &mut path{
                *i+=1;
            }
            path[0]=1;
            path
    }

    pub fn get_row(vec: &Vec<u32>, row: u32) -> Vec<u32>{
        let begin: usize = (row*(row-1)/2).try_into().unwrap();
        let end: usize = (row*(row+1)/2).try_into().unwrap();
        vec[begin..end].to_vec()
    }

    impl Triangle {
        pub fn get_row(&self, row: u32) -> Vec<u8>{
            let begin: usize = (row*(row-1)/2).try_into().unwrap();
            let end: usize = (row*(row+1)/2).try_into().unwrap();
            self.data[begin..end].to_vec()
        }

        pub fn get_data(&self) -> Vec<u8>{
            self.data.clone()
        }

        pub fn get_size(&self) -> u32{
            self.size
        }

        pub fn get_left_sub(&self) -> Triangle{
            let mut new_data = self.get_data();
            for i in 1..self.size{
                new_data.remove((i*(i-1)/2).try_into().unwrap());
            }
            Triangle{
                data: new_data,
                size: self.size-1
            }
        }

        pub fn get_right_sub(&self) -> Triangle{
            let mut new_data = self.get_data();
            let size = self.size;
            let mut offset = 0;
            for i in 1..size+1{
                new_data.remove((i*(i-1)/2-offset).try_into().unwrap());
                offset+=1;
            }
            Triangle{
                data: new_data,
                size: self.size-1
            }
        }

        pub fn print(&self){
            let mut space = String::from("");
            for _ in 0..self.size+1{
                space.push(' ');
            }
            for i  in 1..self.size+1{
                let row = self.get_row(i);
                print!("{}",space);
                for item in row{
                    print!("{} ",item);
                }
                println!();
                space.pop();
            }
        }

        pub fn indexed_print(&self, path: Vec<u32>){
            let mut space = String::from("");
            for _ in 0..self.size+1{
                space.push(' ');
            }
            for i  in 1..self.size+1{
                let row = self.get_row(i);
                print!("{}",space);
                let index: u32 = *path.get(i as usize-1).unwrap();
                let mut j = 1;
                for item in row{
                    if j==index{
                        print!("{} ",Self::get_indexed(&item));
                    }else {
                        print!("{} ",item);
                    }
                    j+=1;
                }
                println!();
                space.pop();
            }
        }

        fn get_indexed<'a>(num: &'a u8) -> &'a str{
            match num{
                1 => "①",
                2 => "②",
                3 => "③",
                4 => "④",
                5 => "⑤",
                6 => "⑥",
                7 => "⑦",
                8 => "⑧",
                9 => "⑨",
                _ => panic!("out of bounds"),
            }
        }

        pub fn solve_recursively(&self) -> Vec<u32>{
            if self.size==2{
                let row_2 = self.get_row(2);
                if row_2.get(1)>row_2.get(0){return vec![1,2];}
                else{return vec![1,1];}
            }else {
                let mut path_1 = self.get_left_sub().solve_recursively();
                println!("Calculating left side:");
                self.get_left_sub().print();//test
                path_1.insert(0, 1);
                let mut path_2 = self.get_right_sub().solve_recursively();
                println!("Calculating right side:");
                self.get_right_sub().print();//test
                path_2.insert(0, 1);
                let result_1 = self.calc_left(&path_1);
                let result_2 = self.calc_right(&path_2);
                if result_1>result_2{
                    path_1
                }else{
                    cancel_offset(path_2)
                }
            }
        }

        pub fn calc_left(&self, path: &Vec<u32>) -> u32{
            let mut result:u32 = 0;
            for i in 1..self.size+1{
                let row = self.get_row(i);
                let index = *path.get(i as usize-1).unwrap();
                result += *row.get(index as usize -1).unwrap() as u32;
            }
            result
        }

        pub fn calc_right(&self, path: &Vec<u32>) -> u32{
            let mut result:u32 = self.data[0].into();
            for i in 2..self.size+1{
                let row = self.get_row(i);
                let index = *path.get(i as usize -1).unwrap();
                result += *row.get(index as usize).unwrap() as u32;
            }
            result
        }

        pub fn solve_dynamically(&self) -> Vec<u32>{
            let mut sum_table = Vec::new();
            sum_table.push(self.get_row(1)[0] as u32);
            for i in 2..self.size+1{
                let row = self.get_row(i);
                for (index, item) in row.iter().enumerate(){
                    let last_row = get_row(&sum_table, i-1);
                    let last_index = row.len()-1;
                    if index==0{
                        sum_table.push(last_row[0]+*item as u32)
                    }else if index==last_index {
                        sum_table.push(last_row[last_index-1]+*item as u32)
                    }else{
                        let prev = last_row[index-1];
                        let next = last_row[index];
                        let max = if prev>next {prev} else {next};
                        sum_table.push(max+*item as u32);
                    }
                }
            }
            let last_row = get_row(&sum_table, self.size);
            println!("{:?}",last_row);//test
            let mut path = Vec::new();
            let mut max = last_row[0];
            let mut max_index = 0;
            for (index, item) in last_row.iter().enumerate(){
                if *item > max{
                    max = *item;
                    max_index = index;
                }
            }
            max_index+=1;
            println!("{}",max_index);//test
            path.push(max_index as u32);
            println!("retracing");
            for j in 1..self.size{
                let i = self.size-j;
                let row = get_row(&sum_table, i);
                if max_index==1{
                    path.push(1);
                }else if max_index==(i+1).try_into().unwrap(){
                    max_index-=1;
                    path.push(max_index.try_into().unwrap());
                }else{
                    let prev = row[max_index-2];
                    let next = row[max_index-1];
                    if prev>next{
                        max_index-=1;
                    }
                    path.push(max_index as u32);
                }
                println!("{:?}",path);
            }
            path.into_iter().rev().collect()
        }
    }
}