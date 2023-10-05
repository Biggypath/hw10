fn main() {
    let data = [
        "<--",
        "#####",
        "<==",
    ]
    .iter()
    .map(|v| v.to_string())
    .collect::<Vec<String>>();

    println!("{:?}", hcat(data.clone(), data[..2].to_vec()));
}

fn vflip(img: Vec<String>) -> Vec<String> {
    let mut result = Vec::new();
    if img.is_empty() {
        return Vec::new()
    }
    for i in img.iter().rev() {
        result.push(i.clone());
    }
    result
}

fn hflip(img: Vec<String>) -> Vec<String> {
    let mut result = Vec::new();
    if img.is_empty() {
        return Vec::new();
    }
    let mut max = 0;
    for length in img.iter() {
        if max < length.len(){
            max = length.len()
        }
    }

    for i in img.iter() {
        let mut stri = String::new();
        let mut count = 0;

        for c in i.chars().rev() {
            stri.push(c);
            count += 1;
        }
        while count < max {
            stri.insert(0, ' ');
            count += 1
        } 
        
        result.push(stri.clone());
    }
    result
}

#[test]
fn test_img_flip() {
    let emp = ["", "", "", ""].iter().map(|v| v.to_string()).collect::<Vec<String>>();
    assert_eq!(vflip(emp.clone()), ["", "", "", ""]);
    assert_eq!(hflip(emp.clone()), ["", "", "", ""]);

    let data = [
        "<--",
        "#####",
        "<==",
    ]
    .iter()
    .map(|v| v.to_string())
    .collect::<Vec<String>>();

    assert_eq!(
        vflip(data.clone()),
        [
            "<==".to_string(),
            "#####".to_string(),
            "<--".to_string(),
        ]
    );
    
    assert_eq!(
        hflip(data.clone()),
        [
            "  --<".to_string(),
            "#####".to_string(),
            "  ==<".to_string(),
        ]
    );
}


fn vcat(img1: Vec<String>, img2: Vec<String>) -> Vec<String> {
    let mut result = Vec::new();
    if img1.is_empty() && img2.is_empty() {
        return Vec::new()
    }
    
    for i in img1.iter() {
        result.push(i.to_string());
        
    }
    for j in img2.iter() {
        result.push(j.to_string());
        
    }
    result
}

fn hcat(mut img1: Vec<String>, img2: Vec<String>) -> Vec<String> {
    if img1.is_empty() && img2.is_empty() {
        return vec!["".to_string(); 0];
    }
    let mut result = Vec::new();
    let mut string1 = Vec::new();
    let mut max = 0;
    for length in img1.iter() {
        if max < length.len(){
            max = length.len()
        }
    }
    
    while img1.len() < img2.len() {
        img1.push("".to_string());
    }

    let mut j = 0;
    for i in img1.iter() {
        let mut str1 = String::new();
        let mut count = 0;
        
        for c in i.chars() {
            str1.push(c);
            count += 1;
        }

        if (j >= img2.len()) && (img1.len() >= img2.len()) { 
            string1.push(str1.clone());
            j += 1;
            continue; 
        }
        else {
            while count < max {
                str1.push(' ');
                count += 1
            }
            string1.push(str1.clone());
            j += 1;     
        }

    }
    let max_len = string1.len().max(img2.len());
    
    for i in 0..max_len {
        let mut concatenated = String::new();
        if i < string1.len() {
            concatenated.push_str(&string1[i]);
        }
        if i < img2.len() {
            concatenated.push_str(&img2[i]);
        }
        result.push(concatenated);
    }

    result
}   


#[test]
fn test_img_cat() {
    let emp = ["".to_string(); 0]; 
    assert_eq!(vcat(emp.to_vec(), emp.to_vec()), [""; 0]); 
    assert_eq!(hcat(emp.to_vec(), emp.to_vec()), [""; 0]);

    let data = [ "<--","#####","<=="]
    .map(|v| v.to_string()); 
    assert_eq!(vcat(emp.to_vec(), data.to_vec()), data); 
    assert_eq!(vcat(data.to_vec(), emp.to_vec()), data);

    assert_eq!(vcat(data.to_vec(), data.to_vec()), ["<--", "#####", "<==", "<--", "#####", "<=="]);
    assert_eq!(hcat(data.to_vec(), data[..2].to_vec()), ["<--  <--", "##########", "<=="]); 
    assert_eq!(hcat(data[..2].to_vec(), data.to_vec()), [ "<--  <--","##########","     <=="]); 

}

